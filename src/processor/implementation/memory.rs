use super::Processor;
use crate::{in_range, mem, reg};

use super::EmulationError;
use crate::instructions::MemoryInstruction;

impl Processor {
    pub(super) fn execute_memory(&mut self, op: MemoryInstruction) -> Result<(), EmulationError> {
        self.flags.unset();
        match op {
            MemoryInstruction::Load(z, y) => {
                in_range![REG_COUNT; z, y];
                in_range![RAM_SIZE; reg![self; y]];
                reg![self; z] = mem![self; reg![self; y]];
                self.flags.zero = reg![self; z] == 0;
                self.flags.sign = reg![self; z] & 0x8000 != 0;
                Ok(())
            }
            MemoryInstruction::Store(x, y) => {
                in_range![REG_COUNT; x, y];
                in_range![RAM_SIZE; reg![self; y]];
                mem![self; reg![self; y]] = reg![self; x];
                self.flags.zero = reg![self; x] == 0;
                self.flags.sign = reg![self; x] & 0x8000 != 0;
                Ok(())
            }
        }
    }
}
