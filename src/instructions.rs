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

#[allow(unused)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DebugInstruction {
    SetRegister(u8, u16),
    SetFlagZero(bool),
    SetFlagSign(bool),
    SetFlagCarry(bool),
    SetMemory(u16, u16),
    Breakpoint(u16),
    Halt,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Instruction {
    ALU(ALUInstruction),
    Memory(MemoryInstruction),
    ControlFlow(ControlFlowInstruction),
    Debug(DebugInstruction),
    #[default]
    NoOperation,
}

#[macro_export]
macro_rules! op {
    (alu $n:ident $($r:literal),*) => {
        crate::instructions::Instruction::ALU(crate::instructions::ALUInstruction::$n($($r),*))
    };
    (mem $n:ident $($r:literal),*) => {
        crate::instructions::Instruction::Memory(crate::instructions::MemoryInstruction::$n($($r),*))
    };
    (cnt $n:ident $($r:literal),*) => {
        crate::instructions::Instruction::ControlFlow(crate::instructions::ControlFlowInstruction::$n($($r),*))
    };
    (dbg $n:ident $($r:literal),*) => {
        crate::instructions::Instruction::Debug(crate::instructions::DebugInstruction::$n($($r),*))
    };
    (nop) => {
        crate::instructions::Instruction::NoOperation
    };
    (mov $z:literal, $x:literal) => {
        op![alu Move $z, $x]
    };
    (add $z:literal, $x:literal, $y:literal) => {
        op![alu Add $z, $x, $y]
    };
    (sub $z:literal, $x:literal, $y:literal) => {
        op![alu Subtract $z, $x, $y]
    };
    (and $z:literal, $x:literal, $y:literal) => {
        op![alu LogicalAnd $z, $x, $y]
    };
    (or $z:literal, $x:literal, $y:literal) => {
        op![alu LogicalOr $z, $x, $y]
    };
    (inc $z:literal, $x:literal) => {
        op![alu Increment $z, $x]
    };
    (dec $z:literal, $x:literal) => {
        op![alu Decrement $z, $x]
    };
    (shl $z:literal, $x:literal) => {
        op![alu LShiftLeft $z, $x]
    };
    (shr $z:literal, $x:literal) => {
        op![alu LShiftRight $z, $x]
    };
    (ashl $z:literal, $x:literal) => {
        op![alu AShiftLeft $z, $x]
    };
    (ashr $z:literal, $x:literal) => {
        op![alu AShiftRight $z, $x]
    };
    (ld $z:literal, $y:literal) => {
        op![mem Load $z, $y]
    };
    (st $x:literal, $y:literal) => {
        op![mem Store $x, $y]
    };
    (jmp $a:literal) => {
        op![cnt Jump $a]
    };
    (jmpz $a:literal) => {
        op![cnt JumpZero $a]
    };
    (jmps $a:literal) => {
        op![cnt JumpSign $a]
    };
    (jmpc $a:literal) => {
        op![cnt JumpCarry $a]
    };
    (jmpnz $a:literal) => {
        op![cnt JumpNotZero $a]
    };
    (jmpns $a:literal) => {
        op![cnt JumpNotSign $a]
    };
    (jmpnc $a:literal) => {
        op![cnt JumpNotCarry $a]
    };
    (sreg $r:literal, $v:literal) => {
        op![dbg SetRegister $r, $v]
    };
    (sfz $v:literal) => {
        op![dbg SetFlagZero $v]
    };
    (sfs $v:literal) => {
        op![dbg SetFlagSign $v]
    };
    (sfc $v:literal) => {
        op![dbg SetFlagCarry $v]
    };
    (smem $a:literal, $v:literal) => {
        op![dbg SetMemory $a, $v]
    };
    (brk $a:literal) => {
        op![dbg Breakpoint $a]
    };
    (halt) => {
        crate::instructions::Instruction::Debug(crate::instructions::DebugInstruction::Halt)
    };
}

#[macro_export]
macro_rules! asm {
    ($($instr:tt $($operand:literal),*);*) => {
        [$(
            op![$instr $($operand),*]
        ),*]
    };
}
