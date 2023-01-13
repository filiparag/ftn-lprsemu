mod display;
mod from_str;
mod implementation;
mod r#macro;

pub use implementation::RegisterBoundCheck;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AluInstruction {
    Move(u8, u8),
    Add(u8, u8, u8),
    Subtract(u8, u8, u8),
    LogicalAnd(u8, u8, u8),
    LogicalOr(u8, u8, u8),
    LogicalNot(u8, u8),
    Increment(u8, u8),
    Decrement(u8, u8),
    LShiftLeft(u8, u8),
    LShiftRight(u8, u8),
    AShiftLeft(u8, u8),
    AShiftRight(u8, u8),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MemoryInstruction {
    Load(u8, u8),
    Store(u8, u8),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ControlFlowInstruction {
    Jump(u16),
    JumpZero(u16),
    JumpSign(u16),
    JumpCarry(u16),
    JumpNotZero(u16),
    JumpNotSign(u16),
    JumpNotCarry(u16),
}

#[allow(unused)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DebugInstruction {
    SetRegister(u8, u16),
    SetFlagZero(bool),
    SetFlagSign(bool),
    SetFlagCarry(bool),
    SetMemory(u16, u16),
    Breakpoint(u16),
    Halt,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Instruction {
    Alu(AluInstruction),
    Memory(MemoryInstruction),
    ControlFlow(ControlFlowInstruction),
    #[allow(unused)]
    Debug(DebugInstruction),
    #[default]
    NoOperation,
}
