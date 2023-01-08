use super::Processor;
use crate::in_range;

use super::EmulationError;
use crate::instructions::ControlFlowInstruction;

impl Processor {
    pub(super) fn execute_control_flow(
        &mut self,
        op: ControlFlowInstruction,
    ) -> Result<(), EmulationError> {
        let res = match op {
            ControlFlowInstruction::Jump(addr) => {
                in_range![RAM_SIZE; addr];
                self.program_counter = addr as usize;
                Ok(())
            }
            ControlFlowInstruction::JumpZero(addr) => {
                in_range![RAM_SIZE; addr];
                if self.flags.zero {
                    self.program_counter = addr as usize;
                }
                Ok(())
            }
            ControlFlowInstruction::JumpSign(addr) => {
                in_range![RAM_SIZE; addr];
                if self.flags.sign {
                    self.program_counter = addr as usize;
                }
                Ok(())
            }
            ControlFlowInstruction::JumpCarry(addr) => {
                in_range![RAM_SIZE; addr];
                if self.flags.carry {
                    self.program_counter = addr as usize;
                }
                Ok(())
            }
            ControlFlowInstruction::JumpNotZero(addr) => {
                in_range![RAM_SIZE; addr];
                if !self.flags.zero {
                    self.program_counter = addr as usize;
                }
                Ok(())
            }
            ControlFlowInstruction::JumpNotSign(addr) => {
                in_range![RAM_SIZE; addr];
                if !self.flags.sign {
                    self.program_counter = addr as usize;
                }
                Ok(())
            }
            ControlFlowInstruction::JumpNotCarry(addr) => {
                in_range![RAM_SIZE; addr];
                if !self.flags.carry {
                    self.program_counter = addr as usize;
                }
                Ok(())
            }
        };
        self.flags.unset();
        res
    }
}
