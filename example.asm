#!/usr/bin/env lprsemu

/* Signed multiplication example */

.data
    0x000               // Result will be stored here
    5, -6               // Multiplication factors

.text
load:
    inc   R0, R0        // Set index to 1
    ld    R1, R0        // Load memory at index 1 (first factor)
    inc   R0, R0        // Set index to 2
    ld    R2, R0        // Load memory at index 2 (second factor)
    sub   R0, R0, R0    // Prepare accumulator
loop:
    add   R0, R0, R1    // Add first factor to accumulator
    dec   R2, R2        // Decrease repetition counter (second factor)
    jmpnz loop          // Repeat if counter is greater than zero
store:
    st    R0, R2        // Store result to memory
end:
    jmp end             // Loop forever
