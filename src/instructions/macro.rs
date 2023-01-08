#[macro_export]
macro_rules! op {
    (alu $n:ident $($r:tt),*) => {
        $crate::instructions::Instruction::Alu($crate::instructions::AluInstruction::$n($($r),*))
    };
    (mem $n:ident $($r:tt),*) => {
        $crate::instructions::Instruction::Memory($crate::instructions::MemoryInstruction::$n($($r),*))
    };
    (cnt $n:ident $($r:tt),*) => {
        $crate::instructions::Instruction::ControlFlow($crate::instructions::ControlFlowInstruction::$n($($r),*))
    };
    (dbg $n:ident $($r:tt),*) => {
        $crate::instructions::Instruction::Debug($crate::instructions::DebugInstruction::$n($($r),*))
    };
    (nop) => {
        $crate::instructions::Instruction::NoOperation
    };
    (mov $z:tt, $x:tt) => {
        $crate::op![alu Move $z, $x]
    };
    (add $z:tt, $x:tt, $y:tt) => {
        $crate::op![alu Add $z, $x, $y]
    };
    (sub $z:tt, $x:tt, $y:tt) => {
        $crate::op![alu Subtract $z, $x, $y]
    };
    (and $z:tt, $x:tt, $y:tt) => {
        $crate::op![alu LogicalAnd $z, $x, $y]
    };
    (or $z:tt, $x:tt, $y:tt) => {
        $crate::op![alu LogicalOr $z, $x, $y]
    };
    (inc $z:tt, $x:tt) => {
        $crate::op![alu Increment $z, $x]
    };
    (dec $z:tt, $x:tt) => {
        $crate::op![alu Decrement $z, $x]
    };
    (shl $z:tt, $x:tt) => {
        $crate::op![alu LShiftLeft $z, $x]
    };
    (shr $z:tt, $x:tt) => {
        $crate::op![alu LShiftRight $z, $x]
    };
    (ashl $z:tt, $x:tt) => {
        $crate::op![alu AShiftLeft $z, $x]
    };
    (ashr $z:tt, $x:tt) => {
        $crate::op![alu AShiftRight $z, $x]
    };
    (ld $z:tt, $y:tt) => {
        $crate::op![mem Load $z, $y]
    };
    (st $x:tt, $y:tt) => {
        $crate::op![mem Store $x, $y]
    };
    (jmp $a:tt) => {
        $crate::op![cnt Jump $a]
    };
    (jmpz $a:tt) => {
        $crate::op![cnt JumpZero $a]
    };
    (jmps $a:tt) => {
        $crate::op![cnt JumpSign $a]
    };
    (jmpc $a:tt) => {
        $crate::op![cnt JumpCarry $a]
    };
    (jmpnz $a:tt) => {
        $crate::op![cnt JumpNotZero $a]
    };
    (jmpns $a:tt) => {
        $crate::op![cnt JumpNotSign $a]
    };
    (jmpnc $a:tt) => {
        $crate::op![cnt JumpNotCarry $a]
    };
    (sreg $r:tt, $v:tt) => {
        $crate::op![dbg SetRegister $r, $v]
    };
    (sfz $v:tt) => {
        $crate::op![dbg SetFlagZero $v]
    };
    (sfs $v:tt) => {
        $crate::op![dbg SetFlagSign $v]
    };
    (sfc $v:tt) => {
        $crate::op![dbg SetFlagCarry $v]
    };
    (smem $a:tt, $v:tt) => {
        $crate::op![dbg SetMemory $a, $v]
    };
    (brk $a:tt) => {
        $crate::op![dbg Breakpoint $a]
    };
    (halt) => {
        $crate::instructions::Instruction::Debug($crate::instructions::DebugInstruction::Halt)
    };
}

#[macro_export]
macro_rules! asm {
    ($($instr:tt $($operand:tt),*);*) => {
        [$(
            $crate::op![$instr $($operand),*]
        ),*]
    };
}
