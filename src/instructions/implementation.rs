use super::{AluInstruction, ControlFlowInstruction, Instruction, MemoryInstruction};

macro_rules! in_range {
    ($thresh:ident; $($v:expr),*) => {
        $(($v as usize) < ($crate::processor::$thresh as usize))&&*
    };
}

pub trait RegisterBoundCheck {
    fn reg_bound_check(&self) -> bool;
}

impl RegisterBoundCheck for Instruction {
    fn reg_bound_check(&self) -> bool {
        match *self {
            Self::Alu(i) => i.reg_bound_check(),
            Self::Memory(i) => i.reg_bound_check(),
            Self::ControlFlow(i) => i.reg_bound_check(),
            _ => true,
        }
    }
}

impl RegisterBoundCheck for AluInstruction {
    fn reg_bound_check(&self) -> bool {
        match *self {
            Self::Move(a, b) => in_range!(REG_COUNT; a, b),
            Self::Add(a, b, c) => in_range!(REG_COUNT; a, b, c),
            Self::Subtract(a, b, c) => in_range!(REG_COUNT; a, b, c),
            Self::LogicalAnd(a, b, c) => in_range!(REG_COUNT; a, b, c),
            Self::LogicalOr(a, b, c) => in_range!(REG_COUNT; a, b, c),
            Self::LogicalNot(a, b) => in_range!(REG_COUNT; a, b),
            Self::Increment(a, b) => in_range!(REG_COUNT; a, b),
            Self::Decrement(a, b) => in_range!(REG_COUNT; a, b),
            Self::LShiftLeft(a, b) => in_range!(REG_COUNT; a, b),
            Self::LShiftRight(a, b) => in_range!(REG_COUNT; a, b),
            Self::AShiftLeft(a, b) => in_range!(REG_COUNT; a, b),
            Self::AShiftRight(a, b) => in_range!(REG_COUNT; a, b),
        }
    }
}

impl RegisterBoundCheck for MemoryInstruction {
    fn reg_bound_check(&self) -> bool {
        match *self {
            Self::Load(a, b) => in_range!(REG_COUNT; a, b),
            Self::Store(a, b) => in_range!(REG_COUNT; a, b),
        }
    }
}

impl RegisterBoundCheck for ControlFlowInstruction {
    fn reg_bound_check(&self) -> bool {
        match *self {
            Self::Jump(a) => in_range!(ROM_SIZE; a),
            Self::JumpZero(a) => in_range!(ROM_SIZE; a),
            Self::JumpSign(a) => in_range!(ROM_SIZE; a),
            Self::JumpCarry(a) => in_range!(ROM_SIZE; a),
            Self::JumpNotZero(a) => in_range!(ROM_SIZE; a),
            Self::JumpNotSign(a) => in_range!(ROM_SIZE; a),
            Self::JumpNotCarry(a) => in_range!(ROM_SIZE; a),
        }
    }
}

impl ControlFlowInstruction {
    pub fn get_address(&self) -> u16 {
        match *self {
            ControlFlowInstruction::Jump(a) => a,
            ControlFlowInstruction::JumpZero(a) => a,
            ControlFlowInstruction::JumpSign(a) => a,
            ControlFlowInstruction::JumpCarry(a) => a,
            ControlFlowInstruction::JumpNotZero(a) => a,
            ControlFlowInstruction::JumpNotSign(a) => a,
            ControlFlowInstruction::JumpNotCarry(a) => a,
        }
    }
}
