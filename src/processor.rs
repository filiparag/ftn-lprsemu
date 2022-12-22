use crate::instructions::Instruction;

#[derive(Debug)]
struct FlagRegisters {
    zero: bool,
    sign: bool,
    carry: bool,
}

pub struct Processor {
    rom: [Instruction; 256],
    ram: [u16; 256],
    registers: [u16; 8],
    flags: FlagRegisters,
    program_counter: usize,
    runtime_counter: usize,
}
