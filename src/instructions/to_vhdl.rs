use crate::instructions::{AluInstruction, ControlFlowInstruction, Instruction, MemoryInstruction};

const EMPTY_REG: u8 = 0;

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
            Self::Move(rz, rx) => format!("000000{}", encode_registers(rz, rx, EMPTY_REG)),
            Self::Add(rz, rx, ry) => format!("000001{}", encode_registers(rz, rx, ry)),
            Self::Subtract(rz, rx, ry) => format!("000010{}", encode_registers(rz, rx, ry)),
            Self::LogicalAnd(rz, rx, ry) => format!("000011{}", encode_registers(rz, rx, ry)),
            Self::LogicalOr(rz, rx, ry) => format!("000100{}", encode_registers(rz, rx, ry)),
            Self::LogicalNot(rz, rx) => format!("000101{}", encode_registers(rz, rx, EMPTY_REG)),
            Self::Increment(rz, rx) => format!("000110{}", encode_registers(rz, rx, EMPTY_REG)),
            Self::Decrement(rz, rx) => format!("000111{}", encode_registers(rz, rx, EMPTY_REG)),
            Self::LShiftLeft(rz, rx) => format!("001000{}", encode_registers(rz, rx, EMPTY_REG)),
            Self::LShiftRight(rz, rx) => format!("001001{}", encode_registers(rz, rx, EMPTY_REG)),
            Self::AShiftLeft(rz, rx) => format!("001010{}", encode_registers(rz, rx, EMPTY_REG)),
            Self::AShiftRight(rz, rx) => format!("001011{}", encode_registers(rz, rx, EMPTY_REG)),
        }
    }
}

impl ToVhdl for MemoryInstruction {
    fn to_vhdl(&self) -> String {
        match *self {
            Self::Load(rz, ry) => format!("100000{}", encode_registers(rz, EMPTY_REG, ry)),
            Self::Store(rx, ry) => format!("110000{}", encode_registers(EMPTY_REG, rx, ry)),
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
