use crate::instructions::{AluInstruction, ControlFlowInstruction, Instruction, MemoryInstruction};

pub trait ToVhdl {
    fn to_vhdl(&self) -> String;
}

impl ToVhdl for Instruction {
    fn to_vhdl(&self) -> String {
        match *self {
            Self::Alu(ins) => ins.to_vhdl(),
            Self::Memory(ins) => ins.to_vhdl(),
            Self::ControlFlow(ins) => ins.to_vhdl(),
            _ => "".into(),
        }
    }
}

impl ToVhdl for AluInstruction {
    fn to_vhdl(&self) -> String {
        match *self {
            Self::Move(r1, r2) => format!("000000{}", encode_registers(r1, r2, 0)),
            Self::Add(r1, r2, r3) => format!("000001{}", encode_registers(r1, r2, r3)),
            Self::Subtract(r1, r2, r3) => format!("000010{}", encode_registers(r1, r2, r3)),
            Self::LogicalAnd(r1, r2, r3) => format!("000011{}", encode_registers(r1, r2, r3)),
            Self::LogicalOr(r1, r2, r3) => format!("000100{}", encode_registers(r1, r2, r3)),
            Self::LogicalNot(r1, r2) => format!("000101{}", encode_registers(r1, r2, 0)),
            Self::Increment(r1, r2) => format!("000110{}", encode_registers(r1, r2, 0)),
            Self::Decrement(r1, r2) => format!("000111{}", encode_registers(r1, r2, 0)),
            Self::LShiftLeft(r1, r2) => format!("001000{}", encode_registers(r1, r2, 0)),
            Self::LShiftRight(r1, r2) => format!("001001{}", encode_registers(r1, r2, 0)),
            Self::AShiftLeft(r1, r2) => format!("001010{}", encode_registers(r1, r2, 0)),
            Self::AShiftRight(r1, r2) => format!("001011{}", encode_registers(r1, r2, 0)),
        }
    }
}

impl ToVhdl for MemoryInstruction {
    fn to_vhdl(&self) -> String {
        match *self {
            Self::Load(r1, r2) => format!("100000{}", encode_registers(r1, r2, 0)),
            Self::Store(r1, r2) => format!("110000{}", encode_registers(r1, r2, 0)),
        }
    }
}

impl ToVhdl for ControlFlowInstruction {
    fn to_vhdl(&self) -> String {
        match *self {
            Self::Jump(a) => format!("010000{}", encode_address(a)),
            Self::JumpZero(a) => format!("010001{}", encode_address(a)),
            Self::JumpSign(a) => format!("010010{}", encode_address(a)),
            Self::JumpCarry(a) => format!("010011{}", encode_address(a)),
            Self::JumpNotZero(a) => format!("010101{}", encode_address(a)),
            Self::JumpNotSign(a) => format!("010110{}", encode_address(a)),
            Self::JumpNotCarry(a) => format!("010111{}", encode_address(a)),
        }
    }
}

fn encode_registers(r1: u8, r2: u8, r3: u8) -> String {
    format!("{r1:03b}{r2:03b}{r3:03b}")
}

fn encode_address(a: u16) -> String {
    format!("{a:09b}")
}

