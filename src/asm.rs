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
    nop;
    nop;
    nop;
    nop
];
