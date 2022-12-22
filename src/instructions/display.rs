use crate::instructions::{ALUInstruction, ControlFlowInstruction, Instruction, MemoryInstruction};

use super::DebugInstruction;

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ALU(op) => match op {
                ALUInstruction::Move(z, x) => write!(f, "mov   R{}, R{}", z, x),
                ALUInstruction::Add(z, x, y) => write!(f, "add   R{}, R{}, R{}", z, x, y),
                ALUInstruction::Subtract(z, x, y) => write!(f, "sub   R{}, R{}, R{}", z, x, y),
                ALUInstruction::LogicalAnd(z, x, y) => write!(f, "and   R{}, R{}, R{}", z, x, y),
                ALUInstruction::LogicalOr(z, x, y) => write!(f, "or    R{}, R{}, R{}", z, x, y),
                ALUInstruction::LogicalNot(z, x) => write!(f, "not   R{}, R{}", z, x),
                ALUInstruction::Increment(z, x) => write!(f, "inc   R{}, R{}", z, x),
                ALUInstruction::Decrement(z, x) => write!(f, "dec   R{}, R{}", z, x),
                ALUInstruction::LShiftLeft(z, x) => write!(f, "shl   R{}, R{}", z, x),
                ALUInstruction::LShiftRight(z, x) => write!(f, "shr   R{}, R{}", z, x),
                ALUInstruction::AShiftLeft(z, x) => write!(f, "ashl  R{}, R{}", z, x),
                ALUInstruction::AShiftRight(z, x) => write!(f, "ashr  R{}, R{}", z, x),
            },
            Self::Memory(op) => match op {
                MemoryInstruction::Load(z, y) => write!(f, "ld    R{}, R{}", z, y),
                MemoryInstruction::Store(x, y) => write!(f, "st    R{}, R{}", x, y),
            },
            Self::ControlFlow(op) => match op {
                ControlFlowInstruction::Jump(a) => write!(f, "jmp   {}", a),
                ControlFlowInstruction::JumpZero(a) => write!(f, "jmpz  {}", a),
                ControlFlowInstruction::JumpSign(a) => write!(f, "jmps  {}", a),
                ControlFlowInstruction::JumpCarry(a) => write!(f, "jmpc  {}", a),
                ControlFlowInstruction::JumpNotZero(a) => write!(f, "jmpnz {}", a),
                ControlFlowInstruction::JumpNotSign(a) => write!(f, "jmpns {}", a),
                ControlFlowInstruction::JumpNotCarry(a) => write!(f, "jmpnc {}", a),
            },
            Self::Debug(op) => match op {
                DebugInstruction::SetRegister(r, v) => write!(f, "sreg  R{}, {}", r, v),
                DebugInstruction::SetFlagZero(v) => write!(f, "sfz   {}", v),
                DebugInstruction::SetFlagSign(v) => write!(f, "sfs   {}", v),
                DebugInstruction::SetFlagCarry(v) => write!(f, "sfc   {}", v),
                DebugInstruction::SetMemory(a, v) => write!(f, "smem  {}, {}", a, v),
                DebugInstruction::Breakpoint(a) => write!(f, "brk   {}", a),
                DebugInstruction::Halt => write!(f, "halt"),
            },
            Self::NoOperation => write!(f, "nop"),
        }
    }
}
