## About

**LPRSemu** is a simple emulator and debugger for
[LPRS1](https://www.rt-rk.uns.ac.rs/?q=predmeti/e2/lprs-1-logi%C4%8Dko-projektovanje-ra%C4%8Dunarskih-sistema-1)
ISA & CPU. It supports loading programs from
[binary string](./src/asm.rs#L3-L9)
representation or by
[directly writing](./src/asm.rs#L14-L29)
assembly code in Rust using a macro.

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
|   0 | inc   R0, R0
|   1 | ld    R1, R0
|   2 | inc   R0, R0
|   3 | ld    R2, R0
|   4 | sub   R0, R0, R0
|   5 | add   R0, R0, R1
|   6 | dec   R2, R2
|   7 | jmpnz 5 <=
|   8 | st    R0, R2
|   9 | shr   R0, R0 (*)
|  10 | jmpnz 9
|  11 | shl   R1, R1
|  12 | jmpnz 11
| ··· | nop

lprsemu >>
```

## Usage
1) Clone repository from GitHub
    ```ini
    git clone https://github.com/filiparag/ftn-lprsemu && cd ftn-lprsemu
    ```

2) Choose one option from the following:
   - Write assembly code into `ROM_ASM` macro located
     in [`src/asm.rs`](./src/asm.rs#L14-L29), and set `DATA_MEMORY` accordingly.
   - Load binary string into `ROM_BIN` from `oQ` in `instr_rom.vhd`, and
     `sMEM` values from `data_ram.vhd` into `DATA_MEMORY`.  
     After launching the emulator, type `lb` to load the program
     stored in binary string form. By default, code is loaded from the macro.

3) Compile and run the emulator
   ```ini
    cargo run --release
    ```
   _Note_: Release mode provides significantly better performance.

4) Type `h` into the prompt to list all commands  
   _Note_: Empty command defaults to `step`.
   
5) Enjoy debugging!
