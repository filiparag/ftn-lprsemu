use super::Processor;
use crate::{in_range, reg};

use crate::error::EmulationError;
use crate::instructions::AluInstruction;

impl Processor {
    pub(super) fn execute_alu(&mut self, op: AluInstruction) -> Result<(), EmulationError> {
        self.flags.unset();
        match op {
            AluInstruction::Move(z, x) => {
                in_range![REG_COUNT; z, x];
                reg![self; z] = reg![self; x];
                self.flags.zero = reg![self; z] == 0;
                self.flags.sign = reg![self; z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::Add(z, x, y) => {
                in_range![REG_COUNT; z, x, y];
                (reg![self; z], self.flags.carry) = reg![self; x].overflowing_add(reg![self; y]);
                self.flags.zero = reg![self; z] == 0;
                self.flags.sign = reg![self; z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::Subtract(z, x, y) => {
                in_range![REG_COUNT; z, x, y];
                (reg![self; z], self.flags.carry) = reg![self; x].overflowing_sub(reg![self; y]);
                self.flags.zero = reg![self; z] == 0;
                self.flags.sign = reg![self; z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::LogicalAnd(z, x, y) => {
                in_range![REG_COUNT; z, x, y];
                reg![self; z] = reg![self; x] & reg![self; y];
                self.flags.zero = reg![self; z] == 0;
                self.flags.sign = reg![self; z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::LogicalOr(z, x, y) => {
                in_range![REG_COUNT; z, x, y];
                reg![self; z] = reg![self; x] | reg![self; y];
                self.flags.zero = reg![self; z] == 0;
                self.flags.sign = reg![self; z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::LogicalNot(z, x) => {
                in_range![REG_COUNT; z, x];
                reg![self; z] = !reg![self; x];
                self.flags.zero = reg![self; z] == 0;
                self.flags.sign = reg![self; z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::Increment(z, x) => {
                in_range![REG_COUNT; z, x];
                (reg![self; z], self.flags.carry) = reg![self; x].overflowing_add(1);
                self.flags.zero = reg![self; z] == 0;
                self.flags.sign = reg![self; z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::Decrement(z, x) => {
                in_range![REG_COUNT; z, x];
                (reg![self; z], self.flags.carry) = reg![self; x].overflowing_sub(1);
                self.flags.zero = reg![self; z] == 0;
                self.flags.sign = reg![self; z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::LShiftLeft(z, x) => {
                in_range![REG_COUNT; z, x];
                self.flags.carry = reg![self; z] & 0x8000 != 0;
                reg![self; z] = unsafe { reg![self; x].unchecked_shl(1) };
                self.flags.zero = reg![self; z] == 0;
                self.flags.sign = reg![self; z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::LShiftRight(z, x) => {
                in_range![REG_COUNT; z, x];
                self.flags.carry = reg![self; z] & 0x0001 != 0;
                reg![self; z] = unsafe { reg![self; x].unchecked_shr(1) };
                self.flags.zero = reg![self; z] == 0;
                self.flags.sign = reg![self; z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::AShiftLeft(z, x) => {
                in_range![REG_COUNT; z, x];
                self.flags.carry = reg![self; z] & 0x8000 != 0;
                reg![self; z] = unsafe { reg![self; x].unchecked_shl(1) };
                self.flags.zero = reg![self; z] == 0;
                self.flags.sign = reg![self; z] & 0x8000 != 0;
                Ok(())
            }
            AluInstruction::AShiftRight(z, x) => {
                in_range![REG_COUNT; z, x];
                self.flags.carry = reg![self; z] & 0x0001 != 0;
                let sign_bit = reg![self; z] & 0x8000;
                reg![self; z] = unsafe { reg![self; x].unchecked_shr(1) } | sign_bit;
                self.flags.zero = reg![self; z] == 0;
                self.flags.sign = reg![self; z] & 0x8000 != 0;
                Ok(())
            }
        }
    }
}
