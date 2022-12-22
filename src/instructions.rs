#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALUInstruction {
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

