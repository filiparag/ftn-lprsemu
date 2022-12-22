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

macro_rules! in_range {
    ($thresh:expr; $($v:expr),*) => {
        if $($v as usize >= $thresh as usize)||* {
            return Err(EmulationError::OutOfRange);
        }
    };
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
    pub fn clear_breakpoints(&mut self) -> &mut Self {
        self.breakpoints = [false; ROM_SIZE];
        self
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
    pub fn tick(&mut self) -> Result<bool, EmulationError> {
        if self.program_counter >= ROM_SIZE {
            return Ok(false);
        }
        let current_counter = self.program_counter;
        self.tick_op(self.rom[current_counter])?;
        if self.program_counter == current_counter {
            if self.program_counter < ROM_SIZE - 1 {
                self.program_counter += 1;
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(current_counter < ROM_SIZE - 1)
        }
    }

    fn tick_op(&mut self, op: Instruction) -> Result<(), EmulationError> {
        match op {
            Instruction::Alu(op) => self.execute_alu(op)?,
            Instruction::Memory(op) => self.execute_memory(op)?,
            Instruction::ControlFlow(op) => self.execute_control_flow(op)?,
            Instruction::Debug(op) => self.execute_debug(op)?,
            Instruction::NoOperation => {}
        }
        self.runtime_counter += 1;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn run(&mut self, breakpoints: bool) -> Result<usize, EmulationError> {
        let instruction_count = self.runtime_counter;
        let end = if let Instruction::NoOperation = self.rom[self.last_instruction_address()] {
            if self.last_instruction_address() == 0 {
                return Ok(0);
            }
            self.last_instruction_address() - 1
        } else {
            self.last_instruction_address()
        };

        while self.program_counter <= end {
            if !self.tick()? {
                return Err(EmulationError::StackOverflow);
            };
            if breakpoints && self.breakpoints[self.program_counter] {
                break;
            }
        }
        return Ok(self.runtime_counter - instruction_count);
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

    fn execute_alu(&mut self, op: AluInstruction) -> Result<(), EmulationError> {
        macro_rules! reg {
            ($i:ident) => {
                self.registers[$i as usize]
            };
        }
        self.flags.unset();
        match op {
            AluInstruction::Move(z, x) => {
                in_range![REG_COUNT; z, x];
                reg![z] = reg![x];
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::Add(z, x, y) => {
                in_range![REG_COUNT; z, x, y];
                (reg![z], self.flags.carry) = reg![x].overflowing_add(reg![y]);
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::Subtract(z, x, y) => {
                in_range![REG_COUNT; z, x, y];
                (reg![z], self.flags.carry) = reg![x].overflowing_sub(reg![y]);
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::LogicalAnd(z, x, y) => {
                in_range![REG_COUNT; z, x, y];
                reg![z] = reg![x] & reg![y];
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::LogicalOr(z, x, y) => {
                in_range![REG_COUNT; z, x, y];
                reg![z] = reg![x] | reg![y];
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::LogicalNot(z, x) => {
                in_range![REG_COUNT; z, x];
                reg![z] = !reg![x];
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::Increment(z, x) => {
                in_range![REG_COUNT; z, x];
                (reg![z], self.flags.carry) = reg![x].overflowing_add(1);
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::Decrement(z, x) => {
                in_range![REG_COUNT; z, x];
                (reg![z], self.flags.carry) = reg![x].overflowing_sub(1);
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::LShiftLeft(z, x) => {
                in_range![REG_COUNT; z, x];
                self.flags.carry = reg![z] & 0x8000 != 0;
                reg![z] = unsafe { reg![x].unchecked_shl(1) };
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::LShiftRight(z, x) => {
                in_range![REG_COUNT; z, x];
                self.flags.carry = reg![z] & 0x0001 != 0;
                reg![z] = unsafe { reg![x].unchecked_shr(1) };
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::AShiftLeft(z, x) => {
                in_range![REG_COUNT; z, x];
                self.flags.carry = reg![z] & 0x8000 != 0;
                reg![z] = unsafe { reg![x].unchecked_shl(1) };
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::AShiftRight(z, x) => {
                in_range![REG_COUNT; z, x];
                self.flags.carry = reg![z] & 0x0001 != 0;
                let sign_bit = reg![z] & 0x8000;
                reg![z] = unsafe { reg![x].unchecked_shr(1) } | sign_bit;
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
        }
    }

    fn execute_memory(&mut self, op: MemoryInstruction) -> Result<(), EmulationError> {
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
                in_range![REG_COUNT; z, y];
                in_range![RAM_SIZE; reg![y]];
                reg![z] = mem![reg![y]];
                Ok(())
            }
            MemoryInstruction::Store(x, y) => {
                in_range![REG_COUNT; x, y];
                in_range![RAM_SIZE; reg![y]];
                mem![reg![y]] = reg![x];
                Ok(())
            }
        }
    }

    fn execute_control_flow(&mut self, op: ControlFlowInstruction) -> Result<(), EmulationError> {
        let res = match op {
            ControlFlowInstruction::Jump(addr) => {
                in_range![RAM_SIZE; addr];
                self.program_counter = addr as usize;
                Ok(())
            }
            ControlFlowInstruction::JumpZero(addr) => {
                in_range![RAM_SIZE; addr];
                if self.flags.zero {
                    self.program_counter = addr as usize;
                }
                Ok(())
            }
            ControlFlowInstruction::JumpSign(addr) => {
                in_range![RAM_SIZE; addr];
                if self.flags.sign {
                    self.program_counter = addr as usize;
                }
                Ok(())
            }
            ControlFlowInstruction::JumpCarry(addr) => {
                in_range![RAM_SIZE; addr];
                if self.flags.carry {
                    self.program_counter = addr as usize;
                }
                Ok(())
            }
            ControlFlowInstruction::JumpNotZero(addr) => {
                in_range![RAM_SIZE; addr];
                if !self.flags.zero {
                    self.program_counter = addr as usize;
                }
                Ok(())
            }
            ControlFlowInstruction::JumpNotSign(addr) => {
                in_range![RAM_SIZE; addr];
                if !self.flags.sign {
                    self.program_counter = addr as usize;
                }
                Ok(())
            }
            ControlFlowInstruction::JumpNotCarry(addr) => {
                in_range![RAM_SIZE; addr];
                if !self.flags.carry {
                    self.program_counter = addr as usize;
                }
                Ok(())
            }
        };
        self.flags.unset();
        res
    }

    fn execute_debug(&mut self, op: DebugInstruction) -> Result<(), EmulationError> {
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
                in_range![REG_COUNT; z];
                reg![z] = v;
                Ok(())
            }
            DebugInstruction::SetFlagZero(v) => {
                self.flags.zero = v;
                Ok(())
            }
            DebugInstruction::SetFlagSign(v) => {
                self.flags.sign = v;
                Ok(())
            }
            DebugInstruction::SetFlagCarry(v) => {
                self.flags.carry = v;
                Ok(())
            }
            DebugInstruction::SetMemory(addr, v) => {
                in_range![RAM_SIZE; addr];
                mem![addr] = v;
                Ok(())
            }
            DebugInstruction::Breakpoint(addr) => {
                in_range![RAM_SIZE; addr];
                self.breakpoints[addr as usize] = true;
                Ok(())
            }
            DebugInstruction::Halt => {
                self.program_counter = ROM_SIZE;
                Ok(())
            }
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
