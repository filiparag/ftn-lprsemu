use super::{DisplayRadix, DisplaySigned, FlagRegisters, Processor, RAM_SIZE, REG_COUNT, ROM_SIZE};
use crate::error::EmulationError;
use crate::instructions::{
    AluInstruction, ControlFlowInstruction, DebugInstruction, Instruction, MemoryInstruction,
};

impl std::fmt::Display for Processor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_registers(f)?;
        self.print_flags(f)?;
        writeln!(f, "Program counter: {}", self.program_counter)?;
        writeln!(f, "Runtime counter: {}", self.runtime_counter)?;
        self.print_ram(f)?;
        self.print_rom(f)?;
        Ok(())
    }
}

impl Processor {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Processor {
            rom: [Instruction::default(); ROM_SIZE],
            ram: [0; RAM_SIZE],
            registers: [0; REG_COUNT],
            flags: FlagRegisters::default(),
            program_counter: 0,
            runtime_counter: 0,
            breakpoints: [false; ROM_SIZE],
            radix: DisplayRadix::Decimal(DisplaySigned::Unsigned),
        }
    }

    #[allow(dead_code)]
    pub fn load_rom(&mut self, instructions: &[Instruction]) -> &mut Self {
        self.clear_rom();
        self.rom[0..instructions.len()].copy_from_slice(instructions);
        self
    }

    #[allow(dead_code)]
    pub fn clear_rom(&mut self) {
        self.rom
            .iter_mut()
            .for_each(|op| *op = Instruction::NoOperation);
    }

    #[allow(dead_code)]
    pub fn load_rom_str(&mut self, instructions: &[&str]) -> Result<&mut Self, EmulationError> {
        for (i, op) in instructions.iter().enumerate() {
            self.rom[i] = op.parse()?;
        }
        Ok(self)
    }

    #[allow(dead_code)]
    pub fn load_ram(&mut self, data: &[u16]) -> &mut Self {
        self.clear_ram();
        self.ram[0..data.len()].copy_from_slice(data);
        self
    }

    fn clear_ram(&mut self) {
        self.ram.iter_mut().for_each(|cell| *cell = 0);
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.registers = [0; REG_COUNT];
        self.flags = FlagRegisters::default();
        self.program_counter = 0;
    }

    #[allow(dead_code)]
    pub fn set_radix(&mut self, radix: DisplayRadix) {
        self.radix = radix;
    }

    #[allow(dead_code)]
    pub fn tick(&mut self) -> bool {
        if self.program_counter >= ROM_SIZE {
            return false;
        }
        let current_counter = self.program_counter;
        self.tick_op(self.rom[current_counter]);
        if self.program_counter == current_counter {
            if self.program_counter < ROM_SIZE - 1 {
                self.program_counter += 1;
                true
            } else {
                false
            }
        } else {
            current_counter < ROM_SIZE - 1
        }
    }

    fn tick_op(&mut self, op: Instruction) {
        match op {
            Instruction::Alu(op) => self.execute_alu(op),
            Instruction::Memory(op) => self.execute_memory(op),
            Instruction::ControlFlow(op) => self.execute_control_flow(op),
            Instruction::Debug(op) => self.execute_debug(op),
            Instruction::NoOperation => {}
        }
        self.runtime_counter += 1;
    }

    #[allow(dead_code)]
    pub fn run(&mut self, breakpoints: bool) -> usize {
        let instruction_count = self.runtime_counter;
        let end = if let Instruction::NoOperation = self.rom[self.last_instruction_address()] {
            if self.last_instruction_address() == 0 {
                return 0;
            }
            self.last_instruction_address() - 1
        } else {
            self.last_instruction_address()
        };

        while self.program_counter <= end {
            if !self.tick() {
                eprintln!("Stack overflow!");
                break;
            };
            if breakpoints && self.breakpoints[self.program_counter] {
                break;
            }
        }
        return self.runtime_counter - instruction_count;
    }

    #[allow(dead_code)]
    pub fn toggle_breakpoint(&mut self, line: usize) -> bool {
        if line > self.rom.len() {
            false
        } else {
            self.breakpoints[line] = !self.breakpoints[line];
            self.breakpoints[line]
        }
    }

    #[allow(dead_code)]
    pub fn program_counter_jump(&mut self, line: usize) -> bool {
        if line > self.rom.len() {
            false
        } else {
            self.program_counter = line;
            true
        }
    }

    fn execute_alu(&mut self, op: AluInstruction) {
        macro_rules! reg {
            ($i:ident) => {
                self.registers[$i as usize]
            };
        }
        self.flags.unset();
        match op {
            AluInstruction::Move(z, x) => {
                reg![z] = reg![x];
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            AluInstruction::Add(z, x, y) => {
                (reg![z], self.flags.carry) = reg![x].overflowing_add(reg![y]);
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            AluInstruction::Subtract(z, x, y) => {
                (reg![z], self.flags.carry) = reg![x].overflowing_sub(reg![y]);
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            AluInstruction::LogicalAnd(z, x, y) => {
                reg![z] = reg![x] & reg![y];
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            AluInstruction::LogicalOr(z, x, y) => {
                reg![z] = reg![x] | reg![y];
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            AluInstruction::LogicalNot(z, x) => {
                reg![z] = !reg![x];
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            AluInstruction::Increment(z, x) => {
                (reg![z], self.flags.carry) = reg![x].overflowing_add(1);
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            AluInstruction::Decrement(z, x) => {
                (reg![z], self.flags.carry) = reg![x].overflowing_sub(1);
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            AluInstruction::LShiftLeft(z, x) => {
                self.flags.carry = reg![z] & 0x8000 != 0;
                reg![z] = unsafe { reg![x].unchecked_shl(1) };
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            AluInstruction::LShiftRight(z, x) => {
                self.flags.carry = reg![z] & 0x0001 != 0;
                reg![z] = unsafe { reg![x].unchecked_shr(1) };
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            AluInstruction::AShiftLeft(z, x) => {
                self.flags.carry = reg![z] & 0x8000 != 0;
                reg![z] = unsafe { reg![x].unchecked_shl(1) };
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
            }
            AluInstruction::AShiftRight(z, x) => {
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

    fn execute_debug(&mut self, op: DebugInstruction) {
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
            DebugInstruction::SetRegister(z, v) => {
                reg![z] = v;
            }
            DebugInstruction::SetFlagZero(v) => {
                self.flags.zero = v;
            }
            DebugInstruction::SetFlagSign(v) => {
                self.flags.sign = v;
            }
            DebugInstruction::SetFlagCarry(v) => {
                self.flags.carry = v;
            }
            DebugInstruction::SetMemory(addr, v) => {
                mem![addr] = v;
            }
            DebugInstruction::Breakpoint(addr) => self.breakpoints[addr as usize] = true,
            DebugInstruction::Halt => self.program_counter = ROM_SIZE,
        }
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

    fn print_registers(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Registers")?;
        for i in 0..self.registers.len() {
            write!(f, "| R{i}: {} ", self.print_value(self.registers[i]))?;
            if i != 0 && (i + 1) % 4 == 0 {
                writeln!(f, "|")?;
            }
        }
        Ok(())
    }

    fn print_flags(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Flags ")?;
        write!(
            f,
            "[ zero: {:#5} ] [ sign: {:#5} ] [ carry: {:#5} ]",
            self.flags.zero, self.flags.sign, self.flags.carry
        )?;
        writeln!(f)?;
        Ok(())
    }

    fn print_ram(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Data memory")?;
        let ram_max = self.ram.len()
            - self
                .ram
                .iter()
                .rev()
                .map_while(|&i| if i == 0 { Some(()) } else { None })
                .count();
        for i in 0..ram_max {
            writeln!(f, "| {:#3} | {}", i, self.print_value(self.ram[i]))?;
        }
        if ram_max < self.ram.len() {
            writeln!(f, "| ··· | {}", self.print_value(0))?;
        }
        Ok(())
    }

    fn print_rom(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Program memory")?;
        for i in 0..self.last_instruction_address() {
            write!(f, "| {:#3} | {}", i, self.rom[i])?;
            if self.program_counter == i {
                write!(f, " <=")?;
            }
            if self.breakpoints[i] {
                writeln!(f, " (*)")?;
            } else {
                writeln!(f)?;
            }
        }
        if self.last_instruction_address() < self.rom.len() {
            write!(f, "| ··· | nop")?;
            if self.program_counter >= self.last_instruction_address() {
                writeln!(f, " <=")?;
            } else {
                writeln!(f)?;
            }
        }
        Ok(())
    }

    fn print_value(&self, value: u16) -> String {
        match &self.radix {
            DisplayRadix::Decimal(signed) => match signed {
                DisplaySigned::Unsigned => format!("{value:5}"),
                DisplaySigned::Signed => format!("{:6}", value as i16),
            },
            DisplayRadix::Hexadecimal => format!("{value:#06x}"),
            DisplayRadix::Binary => format!("{value:#018b}"),
        }
    }
}
