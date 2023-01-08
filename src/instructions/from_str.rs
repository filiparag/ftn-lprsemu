use crate::instructions::{AluInstruction, ControlFlowInstruction, Instruction, MemoryInstruction};
use crate::processor::EmulationError;
use std::str::FromStr;

impl FromStr for AluInstruction {
    type Err = EmulationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 15 {
            return Err(Self::Err::InvalidLength);
        }

        if &s[0..2] != "00" {
            return Err(Self::Err::InvalidInstruction);
        }

        let (r1, r2, r3) = parse_registers(&s[6..15])?;

        match &s[2..6] {
            "0000" => Ok(Self::Move(r1, r2)),
            "0001" => Ok(Self::Add(r1, r2, r3)),
            "0010" => Ok(Self::Subtract(r1, r2, r3)),
            "0011" => Ok(Self::LogicalAnd(r1, r2, r3)),
            "0100" => Ok(Self::LogicalOr(r1, r2, r3)),
            "0101" => Ok(Self::LogicalNot(r1, r2)),
            "0110" => Ok(Self::Increment(r1, r2)),
            "0111" => Ok(Self::Decrement(r1, r2)),
            "1000" => Ok(Self::LShiftLeft(r1, r2)),
            "1001" => Ok(Self::LShiftRight(r1, r2)),
            "1010" => Ok(Self::AShiftLeft(r1, r2)),
            "1011" => Ok(Self::AShiftRight(r1, r2)),
            _ => Err(Self::Err::InvalidInstruction),
        }
    }
}

impl FromStr for MemoryInstruction {
    type Err = EmulationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 15 {
            return Err(Self::Err::InvalidLength);
        }

        let (r1, r2, _) = parse_registers(&s[6..15])?;

        match &s[0..6] {
            "100000" => Ok(Self::Load(r1, r2)),
            "110000" => Ok(Self::Store(r1, r2)),
            _ => Err(Self::Err::InvalidInstruction),
        }
    }
}

impl FromStr for ControlFlowInstruction {
    type Err = EmulationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 15 {
            return Err(Self::Err::InvalidLength);
        }

        if &s[0..2] != "01" {
            return Err(Self::Err::InvalidInstruction);
        }

        let addr = parse_address(&s[6..15])?;

        match &s[2..6] {
            "0000" => Ok(Self::Jump(addr)),
            "0001" => Ok(Self::JumpZero(addr)),
            "0010" => Ok(Self::JumpSign(addr)),
            "0011" => Ok(Self::JumpCarry(addr)),
            "0101" => Ok(Self::JumpNotZero(addr)),
            "0110" => Ok(Self::JumpNotSign(addr)),
            "0111" => Ok(Self::JumpNotCarry(addr)),
            _ => Err(Self::Err::InvalidInstruction),
        }
    }
}

impl FromStr for Instruction {
    type Err = EmulationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 15 {
            return Err(Self::Err::InvalidLength);
        }

        match &s[0..2] {
            "00" => Ok(Self::Alu(s.parse::<AluInstruction>()?)),
            "10" | "11" => Ok(Self::Memory(s.parse::<MemoryInstruction>()?)),
            "01" => Ok(Self::ControlFlow(s.parse::<ControlFlowInstruction>()?)),
            _ => Err(Self::Err::InvalidInstruction),
        }
    }
}

fn parse_registers(s: &str) -> Result<(u8, u8, u8), EmulationError> {
    if s.len() != 9 {
        Err(EmulationError::InvalidLength)
    } else {
        let registers: Vec<u8> = (0..=2)
            .filter_map(|i| u8::from_str_radix(&s[i * 3..=i * 3 + 2], 2).ok())
            .collect();
        if registers.len() == 3 {
            Ok((registers[0], registers[1], registers[2]))
        } else {
            Err(EmulationError::BinaryParsing)
        }
    }
}

fn parse_address(s: &str) -> Result<u16, EmulationError> {
    if s.len() != 9 {
        Err(EmulationError::InvalidLength)
    } else if let Ok(addr) = u16::from_str_radix(s, 2) {
        Ok(addr)
    } else {
        Err(EmulationError::BinaryParsing)
    }
}
