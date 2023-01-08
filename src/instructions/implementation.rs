use super::ControlFlowInstruction;

impl ControlFlowInstruction {
    pub fn get_address(&self) -> u16 {
        match *self {
            ControlFlowInstruction::Jump(a) => a,
            ControlFlowInstruction::JumpZero(a) => a,
            ControlFlowInstruction::JumpSign(a) => a,
            ControlFlowInstruction::JumpCarry(a) => a,
            ControlFlowInstruction::JumpNotZero(a) => a,
            ControlFlowInstruction::JumpNotSign(a) => a,
            ControlFlowInstruction::JumpNotCarry(a) => a,
        }
    }
}
