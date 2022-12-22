pub mod display;
pub mod from_str;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALUInstruction {
    Move(u8, u8),
    Add(u8, u8, u8),
    Subtract(u8, u8, u8),
    LogicalAnd(u8, u8, u8),
    LogicalOr(u8, u8, u8),
    LogicalNot(u8, u8),
    Increment(u8, u8),
    Decrement(u8, u8),
    LShiftLeft(u8, u8),
    LShiftRight(u8, u8),
    AShiftLeft(u8, u8),
    AShiftRight(u8, u8),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MemoryInstruction {
    Load(u8, u8),
    Store(u8, u8),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ControlFlowInstruction {
    Jump(u16),
    JumpZero(u16),
    JumpSign(u16),
    JumpCarry(u16),
    JumpNotZero(u16),
    JumpNotSign(u16),
    JumpNotCarry(u16),
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Instruction {
    ALU(ALUInstruction),
    Memory(MemoryInstruction),
    ControlFlow(ControlFlowInstruction),
    #[default]
    NoOperation,
}

#[macro_export]
macro_rules! op {
    (mov $z:literal, $x:literal) => {
        crate::instructions::Instruction::ALU(crate::instructions::ALUInstruction::Move($z, $x))
    };
    (add $z:literal, $x:literal, $y:literal) => {
        Instruction::ALU(ALUInstruction::Add($z, $x, $y))
    };
    (sub $z:literal, $x:literal, $y:literal) => {
        Instruction::ALU(ALUInstruction::Subtract($z, $x, $y))
    };
    (and $z:literal, $x:literal, $y:literal) => {
        Instruction::ALU(ALUInstruction::LogicalAnd($z, $x, $y))
    };
    (or $z:literal, $x:literal, $y:literal) => {
        Instruction::ALU(ALUInstruction::LogicalOr($z, $x, $y))
    };
    (inc $z:literal, $x:literal) => {
        Instruction::ALU(ALUInstruction::Increment($z, $x))
    };
    (dec $z:literal, $x:literal) => {
        Instruction::ALU(ALUInstruction::Decrement($z, $x))
    };
    (shl $z:literal, $x:literal) => {
        Instruction::ALU(ALUInstruction::LShiftLeft($z, $x))
    };
    (shr $z:literal, $x:literal) => {
        Instruction::ALU(ALUInstruction::LShiftRight($z, $x))
    };
    (ashl $z:literal, $x:literal) => {
        Instruction::ALU(ALUInstruction::AShiftLeft($z, $x))
    };
    (ashr $z:literal, $x:literal) => {
        Instruction::ALU(ALUInstruction::AShiftRight($z, $x))
    };
    (ld $z:literal, $y:literal) => {
        Instruction::Memory(MemoryInstruction::Load($z, $y))
    };
    (st $x:literal, $y:literal) => {
        crate::instructions::Instruction::Memory(crate::instructions::MemoryInstruction::Store(
            $x, $y,
        ))
    };
    (jmp $a:literal) => {
        Instruction::ControlFlow(ControlFlowInstruction::Jump($a))
    };
    (jmpz $a:literal) => {
        Instruction::ControlFlow(ControlFlowInstruction::JumpZero($a))
    };
    (jmps $a:literal) => {
        Instruction::ControlFlow(ControlFlowInstruction::JumpSign($a))
    };
    (jmpc $a:literal) => {
        Instruction::ControlFlow(ControlFlowInstruction::JumpCarry($a))
    };
    (jmpnz $a:literal) => {
        Instruction::ControlFlow(ControlFlowInstruction::JumpNotZero($a))
    };
    (jmpns $a:literal) => {
        Instruction::ControlFlow(ControlFlowInstruction::JumpNotSign($a))
    };
    (jmpnc $a:literal) => {
        Instruction::ControlFlow(ControlFlowInstruction::JumpNotCarry($a))
    };
    (nop) => {
        crate::instructions::Instruction::NoOperation
    };
}
