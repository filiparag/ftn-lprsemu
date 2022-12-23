#[macro_export]
macro_rules! op {
    (alu $n:ident $($r:literal),*) => {
        $crate::instructions::Instruction::Alu($crate::instructions::AluInstruction::$n($($r),*))
    };
    (mem $n:ident $($r:literal),*) => {
        $crate::instructions::Instruction::Memory($crate::instructions::MemoryInstruction::$n($($r),*))
    };
    (cnt $n:ident $($r:literal),*) => {
        $crate::instructions::Instruction::ControlFlow($crate::instructions::ControlFlowInstruction::$n($($r),*))
    };
    (dbg $n:ident $($r:literal),*) => {
        $crate::instructions::Instruction::Debug($crate::instructions::DebugInstruction::$n($($r),*))
    };
    (nop) => {
        $crate::instructions::Instruction::NoOperation
    };
    (mov $z:literal, $x:literal) => {
        crate::op![alu Move $z, $x]
    };
    (add $z:literal, $x:literal, $y:literal) => {
        crate::op![alu Add $z, $x, $y]
    };
    (sub $z:literal, $x:literal, $y:literal) => {
        crate::op![alu Subtract $z, $x, $y]
    };
    (and $z:literal, $x:literal, $y:literal) => {
        crate::op![alu LogicalAnd $z, $x, $y]
    };
    (or $z:literal, $x:literal, $y:literal) => {
        crate::op![alu LogicalOr $z, $x, $y]
    };
    (inc $z:literal, $x:literal) => {
        crate::op![alu Increment $z, $x]
    };
    (dec $z:literal, $x:literal) => {
        crate::op![alu Decrement $z, $x]
    };
    (shl $z:literal, $x:literal) => {
        crate::op![alu LShiftLeft $z, $x]
    };
    (shr $z:literal, $x:literal) => {
        crate::op![alu LShiftRight $z, $x]
    };
    (ashl $z:literal, $x:literal) => {
        crate::op![alu AShiftLeft $z, $x]
    };
    (ashr $z:literal, $x:literal) => {
        crate::op![alu AShiftRight $z, $x]
    };
    (ld $z:literal, $y:literal) => {
        crate::op![mem Load $z, $y]
    };
    (st $x:literal, $y:literal) => {
        crate::op![mem Store $x, $y]
    };
    (jmp $a:literal) => {
        crate::op![cnt Jump $a]
    };
    (jmpz $a:literal) => {
        crate::op![cnt JumpZero $a]
    };
    (jmps $a:literal) => {
        crate::op![cnt JumpSign $a]
    };
    (jmpc $a:literal) => {
        crate::op![cnt JumpCarry $a]
    };
    (jmpnz $a:literal) => {
        crate::op![cnt JumpNotZero $a]
    };
    (jmpns $a:literal) => {
        crate::op![cnt JumpNotSign $a]
    };
    (jmpnc $a:literal) => {
        crate::op![cnt JumpNotCarry $a]
    };
    (sreg $r:literal, $v:literal) => {
        crate::op![dbg SetRegister $r, $v]
    };
    (sfz $v:literal) => {
        crate::op![dbg SetFlagZero $v]
    };
    (sfs $v:literal) => {
        crate::op![dbg SetFlagSign $v]
    };
    (sfc $v:literal) => {
        crate::op![dbg SetFlagCarry $v]
    };
    (smem $a:literal, $v:literal) => {
        crate::op![dbg SetMemory $a, $v]
    };
    (brk $a:literal) => {
        crate::op![dbg Breakpoint $a]
    };
    (halt) => {
        $crate::instructions::Instruction::Debug($crate::instructions::DebugInstruction::Halt)
    };
}

#[macro_export]
macro_rules! asm {
    ($($instr:tt $($operand:literal),*);*) => {
        [$(
            crate::op![$instr $($operand),*]
        ),*]
    };
}
