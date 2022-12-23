use super::{DisplayRadix, DisplaySigned, FlagRegisters, Processor, RAM_SIZE, REG_COUNT, ROM_SIZE};
use crate::error::EmulationError;
use crate::instructions::Instruction;

mod alu;
mod control_flow;
mod debug;
mod display;
mod memory;

#[macro_export]
macro_rules! in_range {
    ($thresh:ident; $($v:expr),*) => {
        if $($v as usize >= crate::processor::$thresh as usize)||* {
            return Err(EmulationError::OutOfRange);
        }
    };
}

#[macro_export]
macro_rules! reg {
    ($self:ident; $i:ident) => {
        $self.registers[$i as usize]
    };
}

#[macro_export]
macro_rules! mem {
    ($self:ident; $i:expr) => {
        $self.ram[$i as usize]
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
