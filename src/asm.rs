use crate::{asm, instructions::Instruction, op};

#[allow(dead_code)]
pub const ROM_BIN: [&str; 4] = [
    "000000000000000",
    "000000000000000",
    "000000000000000",
    "000000000000000",
];

#[allow(dead_code)]
pub const DATA_MEMORY: &[u16] = &[0, 5, 6];

#[allow(dead_code)]
pub const ASSEMBLY_CODE: &[Instruction] = &asm![
    inc 0, 0; // set R0 index to 1
    ld 1, 0; // load operand from mem[1]
    inc 0, 0; // set R0 index to 2
    ld 2, 0; // load operand from mem[2]
    sub 0, 0, 0; // zero R0
    add 0, 0, 1; // add R1 operand to R2
    dec 2, 2; // decrement operand R2
    jmpnz 5; // repead addition
    st 0, 2; // store product to mem[0]
    shr 0, 0; // iteratively clear R0
    jmpnz 9;
    shl 1, 1; // iteratively clear R1
    jmpnz 11
];

pub const BENCHMARK: &[Instruction] = &asm![
    smem 0, 0xffff;
    smem 1, 0xffff;
    sub 0, 0, 0;
    sub 1, 1, 1;
    ld 0, 0;
    dec 0, 0;
    jmpz 11;
    st 0, 1;
    dec 0, 0;
    jmpnz 8;
    jmpz 4;
    inc 0, 0;
    ld 0, 0;
    dec 0, 0;
    inc 1, 1;
    st 0, 1;
    inc 0, 0;
    dec 0, 0;
    jmpnz 2;
    nop
];
