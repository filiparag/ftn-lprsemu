.data
0x0000
5, 6

.text
main:
    inc   R0, R0
    ld    R1, R0
    inc   R0, R0
    ld    R2, R0
    sub   R0, R0, R0
loop:
    add   R0, R0, R1
    dec   R2, R2
    jmpnz loop
    st    R0, R2
shift:
    shr   R0, R0
    jmpnz shift
divide:
    shl   R1, R1
    jmpnz divide
