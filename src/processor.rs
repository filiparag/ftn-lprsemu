pub mod flag_registers;
pub mod processor;

use crate::instructions::Instruction;

const ROM_SIZE: usize = 256;
const RAM_SIZE: usize = 256;
const REG_COUNT: usize = 8;

#[derive(Debug)]
struct FlagRegisters {
    zero: bool,
    sign: bool,
    carry: bool,
}

pub struct Processor {
    rom: [Instruction; ROM_SIZE],
    ram: [u16; RAM_SIZE],
    registers: [u16; REG_COUNT],
    flags: FlagRegisters,
    program_counter: usize,
    runtime_counter: usize,
}
