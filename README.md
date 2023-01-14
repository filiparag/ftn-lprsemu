## About

**LPRSemu** is an interactive emulator and debugger for
[LPRS1](https://www.rt-rk.uns.ac.rs/?q=predmeti/e2/lprs-1-logi%C4%8Dko-projektovanje-ra%C4%8Dunarskih-sistema-1)
ISA & CPU.  
**LPRSasm** is an assembler for the reference VHDL imlpementation of the CPU.

**Emulation example**

```
Registers
| R0:    30 | R1:     5 | R2:     0 | R3:     0 |
| R4:     0 | R5:     0 | R6:     0 | R7:     0 |
Flags [ zero: true  ] [ sign: false ] [ carry: false ]
Program counter: 7
Runtime counter: 22
Data memory
|   0 |     0
|   1 |     5
|   2 |     6
| ··· |     0
Program memory
|     | main:
|   0 |     inc   R0, R0
|   1 |     ld    R1, R0
|   2 |     inc   R0, R0
|   3 |     ld    R2, R0
|   4 |     sub   R0, R0, R0
|     | loop:
|   5 |     add   R0, R0, R1
|   6 |     dec   R2, R2
|   7 |     jmpnz 5 (loop) <=
|   8 |     st    R0, R2
|     | shift:
|   9 |     shr   R0, R0 (*)
|  10 |     jmpnz 9 (shift)
|     | divide:
|  11 |     shl   R1, R1
|  12 |     jmpnz 11 (divide)
| ··· | nop

lprsemu >>
```

## Usage

1) Download [latest stabe](https://github.com/filiparag/ftn-lprsemu/releases/latest) binaries for your platform

2) Run the emulator with your assembly code file as the first argument
   ```sh
   ./lprsemu example.asm
   ```

3) Type `h` into the prompt to list all commands  
   _Note_: Empty command defaults to `step`.
   
4) Test and debug your program

5) _Optional_: Assemble it into VHDL using LPRSasm
   ```sh
   ./lprsasm example.asm
   ```
