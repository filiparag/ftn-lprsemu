pub mod flag_registers;
pub mod implementation;

use std::collections::HashMap;

use crate::instructions::Instruction;

pub const ROM_SIZE: usize = 256;
pub const RAM_SIZE: usize = 256;
pub const REG_COUNT: usize = 8;

#[derive(Debug, Default)]
struct FlagRegisters {
    zero: bool,
    sign: bool,
    carry: bool,
}

#[derive(Debug)]
pub enum DisplaySigned {
    Unsigned,
    Signed,
}

#[derive(Debug)]
pub enum DisplayRadix {
    Decimal(DisplaySigned),
    Hexadecimal,
    Binary,
}

pub struct Processor {
    rom: [Instruction; ROM_SIZE],
    ram: [u16; RAM_SIZE],
    ram_initial: [u16; RAM_SIZE],
    registers: [u16; REG_COUNT],
    flags: FlagRegisters,
    program_counter: usize,
    runtime_counter: usize,
    breakpoints: [bool; ROM_SIZE],
    radix: DisplayRadix,
    labels: HashMap<usize, Vec<String>>,
}

#[derive(Clone, Copy, Debug)]
pub enum EmulationError {
    InvalidLength,
    BinaryParsing,
    InvalidInstruction,
    OutOfRange,
    StackOverflow,
}
