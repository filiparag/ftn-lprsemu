use super::Processor;
use crate::{in_range, mem, reg};

use crate::error::EmulationError;
use crate::instructions::DebugInstruction;
use crate::processor::ROM_SIZE;

impl Processor {
    pub(super) fn execute_debug(&mut self, op: DebugInstruction) -> Result<(), EmulationError> {
        macro_rules! reg {
            ($i:ident) => {
                self.registers[$i as usize]
            };
        }
        macro_rules! mem {
            ($i:expr) => {
                self.ram[$i as usize]
            };
        }
        self.flags.unset();
        match op {
            DebugInstruction::SetRegister(z, v) => {
                in_range![REG_COUNT; z];
                reg![z] = v;
                Ok(())
            }
            DebugInstruction::SetFlagZero(v) => {
                self.flags.zero = v;
                Ok(())
            }
            DebugInstruction::SetFlagSign(v) => {
                self.flags.sign = v;
                Ok(())
            }
            DebugInstruction::SetFlagCarry(v) => {
                self.flags.carry = v;
                Ok(())
            }
            DebugInstruction::SetMemory(addr, v) => {
                in_range![RAM_SIZE; addr];
                mem![addr] = v;
                Ok(())
            }
            DebugInstruction::Breakpoint(addr) => {
                in_range![RAM_SIZE; addr];
                self.breakpoints[addr as usize] = true;
                Ok(())
            }
            DebugInstruction::Halt => {
                self.program_counter = ROM_SIZE;
                Ok(())
            }
        }
    }
}
