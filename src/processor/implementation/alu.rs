use super::Processor;
use crate::{in_range, reg};

use crate::error::EmulationError;
use crate::instructions::AluInstruction;

impl Processor {
    pub(super) fn execute_alu(&mut self, op: AluInstruction) -> Result<(), EmulationError> {
        macro_rules! reg {
            ($i:ident) => {
                self.registers[$i as usize]
            };
        }
        self.flags.unset();
        match op {
            AluInstruction::Move(z, x) => {
                in_range![REG_COUNT; z, x];
                reg![z] = reg![x];
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::Add(z, x, y) => {
                in_range![REG_COUNT; z, x, y];
                (reg![z], self.flags.carry) = reg![x].overflowing_add(reg![y]);
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::Subtract(z, x, y) => {
                in_range![REG_COUNT; z, x, y];
                (reg![z], self.flags.carry) = reg![x].overflowing_sub(reg![y]);
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::LogicalAnd(z, x, y) => {
                in_range![REG_COUNT; z, x, y];
                reg![z] = reg![x] & reg![y];
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::LogicalOr(z, x, y) => {
                in_range![REG_COUNT; z, x, y];
                reg![z] = reg![x] | reg![y];
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::LogicalNot(z, x) => {
                in_range![REG_COUNT; z, x];
                reg![z] = !reg![x];
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::Increment(z, x) => {
                in_range![REG_COUNT; z, x];
                (reg![z], self.flags.carry) = reg![x].overflowing_add(1);
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::Decrement(z, x) => {
                in_range![REG_COUNT; z, x];
                (reg![z], self.flags.carry) = reg![x].overflowing_sub(1);
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::LShiftLeft(z, x) => {
                in_range![REG_COUNT; z, x];
                self.flags.carry = reg![z] & 0x8000 != 0;
                reg![z] = unsafe { reg![x].unchecked_shl(1) };
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::LShiftRight(z, x) => {
                in_range![REG_COUNT; z, x];
                self.flags.carry = reg![z] & 0x0001 != 0;
                reg![z] = unsafe { reg![x].unchecked_shr(1) };
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::AShiftLeft(z, x) => {
                in_range![REG_COUNT; z, x];
                self.flags.carry = reg![z] & 0x8000 != 0;
                reg![z] = unsafe { reg![x].unchecked_shl(1) };
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::AShiftRight(z, x) => {
                in_range![REG_COUNT; z, x];
                self.flags.carry = reg![z] & 0x0001 != 0;
                let sign_bit = reg![z] & 0x8000;
                reg![z] = unsafe { reg![x].unchecked_shr(1) } | sign_bit;
                self.flags.zero = reg![z] == 0;
                self.flags.sign = reg![z] & 0x8000 != 0;
                Ok(())
            }
        }
    }
}
