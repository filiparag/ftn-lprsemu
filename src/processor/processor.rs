use super::{FlagRegisters, Processor};
use crate::error::EmulationError;
use crate::instructions::{ALUInstruction, ControlFlowInstruction, Instruction, MemoryInstruction};

impl std::fmt::Display for Processor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Registers")?;
        for i in 0..self.registers.len() {
            write!(f, "\t|    R{}", i)?;
        }
        writeln!(f, "")?;
        for i in 0..self.registers.len() {
            write!(f, "\t| {:#5}", self.registers[i] as i16)?;
        }
        writeln!(f, "")?;
        writeln!(f, "Flags")?;
        writeln!(f, "\tzero: {}", self.flags.zero)?;
        writeln!(f, "\tsign: {}", self.flags.sign)?;
        writeln!(f, "\tcarry: {}", self.flags.carry)?;
        writeln!(f, "Data memory")?;
        let ram_max = self.ram.len()
            - self
                .ram
                .iter()
                .rev()
                .map_while(|&i| if i == 0 { Some(()) } else { None })
                .count();
        for i in 0..ram_max {
            writeln!(f, "\t| {:#3} | {:#5}", i, self.ram[i] as i16)?;
        }
        if ram_max == 0 {
            writeln!(f, "\t<empty>")?;
        } else if ram_max < self.ram.len() {
            writeln!(f, "\t| ... | {:#5}", 0)?;
        }
        writeln!(f, "Program memory")?;
        for i in 0..self.last_instruction_address() {
            writeln!(f, "\t| {:#3} | {}", i, self.rom[i])?;
        }
        if ram_max == 0 {
            writeln!(f, "\t<empty>")?;
        } else if ram_max < self.ram.len() {
            writeln!(f, "\t| ... | nop")?;
        }
        writeln!(f, "Program counter:")?;
        writeln!(f, "\t{}", self.program_counter)?;
        writeln!(f, "Runtime counter:")?;
        writeln!(f, "\t{} ticks", self.runtime_counter)?;
        Ok(())
    }
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            rom: [Instruction::default(); 256],
            ram: [0; 256],
            registers: [0; 8],
            flags: FlagRegisters::default(),
            program_counter: 0,
            runtime_counter: 0,
        }
    }

    pub fn load_rom(&mut self, instructions: &[Instruction]) -> &mut Self {
        self.clear_rom();
        self.rom[0..instructions.len()].copy_from_slice(instructions);
        self
    }

    fn clear_rom(&mut self) {
        self.rom
            .iter_mut()
            .for_each(|op| *op = Instruction::NoOperation);
    }

    pub fn load_rom_str(&mut self, instructions: &[&str]) -> Result<&mut Self, EmulationError> {
        for i in 0..instructions.len() {
            self.rom[i] = instructions[i].parse()?;
        }
        Ok(self)
    }

    pub fn load_ram(&mut self, data: &[u16]) -> &mut Self {
        self.clear_ram();
        self.ram[0..data.len()].copy_from_slice(data);
        self
    }

    fn clear_ram(&mut self) {
        self.ram.iter_mut().for_each(|cell| *cell = 0);
    }

    pub fn tick(&mut self) -> bool {
        self.tick_op(self.rom[self.program_counter]);
        if self.program_counter < 255 {
            self.program_counter += 1;
            true
        } else {
            false
        }
    }

    pub fn run(&mut self) {
        for _ in 0..self.last_instruction_address() {
            if !self.tick() {
                eprintln!("Stack overflow!");
                break;
            };
        }
    }

    pub fn tick_op(&mut self, op: Instruction) {
        match op {
            Instruction::ALU(op) => self.execute_alu(op),
            Instruction::Memory(op) => self.execute_memory(op),
            Instruction::ControlFlow(op) => self.execute_control_flow(op),
            Instruction::NoOperation => {}
        }
        self.runtime_counter += 1;
    }

    fn execute_alu(&mut self, op: ALUInstruction) {
        macro_rules! reg {
            ($i:ident) => {
                self.registers[$i as usize]
            };
        }
        self.flags.unset();
        match op {
            ALUInstruction::Move(z, x) => {
                reg![z] = reg![x];
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            ALUInstruction::Add(z, x, y) => {
                (reg![z], self.flags.carry) = reg![x].overflowing_add(reg![y]);
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            ALUInstruction::Subtract(z, x, y) => {
                (reg![z], self.flags.carry) = reg![x].overflowing_sub(reg![y]);
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            ALUInstruction::LogicalAnd(z, x, y) => {
                reg![z] = reg![x] & reg![y];
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            ALUInstruction::LogicalOr(z, x, y) => {
                reg![z] = reg![x] | reg![y];
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            ALUInstruction::LogicalNot(z, x) => {
                reg![z] = !reg![x];
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            ALUInstruction::Increment(z, x) => {
                (reg![z], self.flags.carry) = reg![x].overflowing_add(1);
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            ALUInstruction::Decrement(z, x) => {
                (reg![z], self.flags.carry) = reg![x].overflowing_sub(1);
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            ALUInstruction::LShiftLeft(z, x) => {
                self.flags.carry = reg![z] & 0x8000 != 0;
                reg![z] = unsafe { reg![x].unchecked_shl(1) };
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            ALUInstruction::LShiftRight(z, x) => {
                self.flags.carry = reg![z] & 0x0001 != 0;
                reg![z] = unsafe { reg![x].unchecked_shr(1) };
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            ALUInstruction::AShiftLeft(z, x) => {
                self.flags.carry = reg![z] & 0x8000 != 0;
                reg![z] = unsafe { reg![x].unchecked_shl(1) };
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            ALUInstruction::AShiftRight(z, x) => {
                self.flags.carry = reg![z] & 0x0001 != 0;
                let sign_bit = reg![z] & 0x8000;
                reg![z] = unsafe { reg![x].unchecked_shr(1) } | sign_bit;
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
        }
    }

    fn execute_memory(&mut self, op: MemoryInstruction) {
        macro_rules! reg {
            ($i:ident) => {
                self.registers[$i as usize]
            };
        }
        macro_rules! mem {
            ($i:expr) => {
                self.ram[$i as usize]
            };
        }
        self.flags.unset();
        match op {
            MemoryInstruction::Load(z, y) => {
                reg![z] = mem![reg![y]];
            }
            MemoryInstruction::Store(x, y) => {
                mem![reg![y]] = reg![x];
            }
        }
    }

    fn execute_control_flow(&mut self, op: ControlFlowInstruction) {
        match op {
            ControlFlowInstruction::Jump(addr) => self.program_counter = addr as usize,
            ControlFlowInstruction::JumpZero(addr) => {
                if self.flags.zero {
                    self.program_counter = addr as usize;
                }
            }
            ControlFlowInstruction::JumpSign(addr) => {
                if self.flags.sign {
                    self.program_counter = addr as usize;
                }
            }
            ControlFlowInstruction::JumpCarry(addr) => {
                if self.flags.carry {
                    self.program_counter = addr as usize;
                }
            }
            ControlFlowInstruction::JumpNotZero(addr) => {
                if !self.flags.zero {
                    self.program_counter = addr as usize;
                }
            }
            ControlFlowInstruction::JumpNotSign(addr) => {
                if !self.flags.sign {
                    self.program_counter = addr as usize;
                }
            }
            ControlFlowInstruction::JumpNotCarry(addr) => {
                if !self.flags.carry {
                    self.program_counter = addr as usize;
                }
            }
        }
        self.flags.unset();
    }

    fn last_instruction_address(&self) -> usize {
        self.rom.len()
            - self
                .rom
                .iter()
                .rev()
                .map_while(|&i| {
                    if i == Instruction::NoOperation {
                        Some(())
                    } else {
                        None
                    }
                })
                .count()
    }
}
