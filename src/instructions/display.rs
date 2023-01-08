use crate::instructions::{
    AluInstruction, ControlFlowInstruction, DebugInstruction, Instruction, MemoryInstruction,
};

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Alu(op) => write!(f, "{op}"),
            Self::Memory(op) => write!(f, "{op}"),
            Self::ControlFlow(op) => write!(f, "{op}"),
            Self::Debug(op) => write!(f, "{op}"),
            Self::NoOperation => write!(f, "nop"),
        }
    }
}

impl std::fmt::Display for AluInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AluInstruction::Move(z, x) => write!(f, "mov   R{z}, R{x}"),
            AluInstruction::Add(z, x, y) => write!(f, "add   R{z}, R{x}, R{y}"),
            AluInstruction::Subtract(z, x, y) => write!(f, "sub   R{z}, R{x}, R{y}"),
            AluInstruction::LogicalAnd(z, x, y) => write!(f, "and   R{z}, R{x}, R{y}"),
            AluInstruction::LogicalOr(z, x, y) => write!(f, "or    R{z}, R{x}, R{y}"),
            AluInstruction::LogicalNot(z, x) => write!(f, "not   R{z}, R{x}"),
            AluInstruction::Increment(z, x) => write!(f, "inc   R{z}, R{x}"),
            AluInstruction::Decrement(z, x) => write!(f, "dec   R{z}, R{x}"),
            AluInstruction::LShiftLeft(z, x) => write!(f, "shl   R{z}, R{x}"),
            AluInstruction::LShiftRight(z, x) => write!(f, "shr   R{z}, R{x}"),
            AluInstruction::AShiftLeft(z, x) => write!(f, "ashl  R{z}, R{x}"),
            AluInstruction::AShiftRight(z, x) => write!(f, "ashr  R{z}, R{x}"),
        }
    }
}

impl std::fmt::Display for MemoryInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MemoryInstruction::Load(z, y) => write!(f, "ld    R{z}, R{y}"),
            MemoryInstruction::Store(x, y) => write!(f, "st    R{x}, R{y}"),
        }
    }
}

impl std::fmt::Display for ControlFlowInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ControlFlowInstruction::Jump(a) => write!(f, "jmp   {a}"),
            ControlFlowInstruction::JumpZero(a) => write!(f, "jmpz  {a}"),
            ControlFlowInstruction::JumpSign(a) => write!(f, "jmps  {a}"),
            ControlFlowInstruction::JumpCarry(a) => write!(f, "jmpc  {a}"),
            ControlFlowInstruction::JumpNotZero(a) => write!(f, "jmpnz {a}"),
            ControlFlowInstruction::JumpNotSign(a) => write!(f, "jmpns {a}"),
            ControlFlowInstruction::JumpNotCarry(a) => write!(f, "jmpnc {a}"),
        }
    }
}

impl std::fmt::Display for DebugInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DebugInstruction::SetRegister(r, v) => write!(f, "sreg  R{r}, {v}"),
            DebugInstruction::SetFlagZero(v) => write!(f, "sfz   {v}"),
            DebugInstruction::SetFlagSign(v) => write!(f, "sfs   {v}"),
            DebugInstruction::SetFlagCarry(v) => write!(f, "sfc   {v}"),
            DebugInstruction::SetMemory(a, v) => write!(f, "smem  {a}, {v}"),
            DebugInstruction::Breakpoint(a) => write!(f, "brk   {a}"),
            DebugInstruction::Halt => write!(f, "halt"),
        }
    }
}
