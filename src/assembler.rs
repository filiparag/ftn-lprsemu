#![feature(unchecked_math)]
#![allow(dead_code)]

use instructions::{Instruction, ToVhdl};
use processor::Processor;
use std::error::Error;
use std::io::Write;
use std::process::ExitCode;

mod asm;
mod instructions;
mod load;
mod parser;
mod processor;

#[macro_use]
extern crate pest_derive;

macro_rules! concat {
    ($vec:ident <= $val:literal $(,$p:expr)*) => {
        write!($vec, $val $(,$p)*).expect("Unexpected formatting error")
    };
    ($vec:ident <= $val:literal $(,$p:expr)* ;) => {
        writeln!($vec, $val $(,$p)*).expect("Unexpected formatting error")
    };
}

fn parse_rom(rom: &[Instruction], comments: bool) -> Vec<u8> {
    let mut vhdl = Vec::new();
    if comments {
        concat!(vhdl <= "-- begin instr_rom.vhd";);
    }
    concat!(vhdl <= "architecture Behavioral of instr_rom is";);
    concat!(vhdl <= "begin";);
    if rom.is_empty() {
        concat!(vhdl <= "    oQ <= \"{:015}\";", 0;);
    } else {
        for (a, ins) in rom.iter().enumerate() {
            if a == 0 {
                concat!(vhdl <= "    oQ <= ");
            } else {
                concat!(vhdl <= "          ");
            }
            concat!(vhdl <= "\"{}\"  when iA = {a} else", ins.to_vhdl(););
        }
        concat!(vhdl <= "          \"{:015}\";", 0;);
    }
    concat!(vhdl <= "end Behavioral;";);
    if comments {
        concat!(vhdl <= "-- end instr_rom.vhd";);
    }
    vhdl
}

fn parse_ram(ram: &[u16], comments: bool) -> Vec<u8> {
    let mut vhdl = Vec::new();
    if comments {
        concat!(vhdl <= "-- begin data_ram.vhd";);
    }
    for (index, value) in ram.iter().enumerate() {
        concat!(vhdl <= "sMEM({index}) <= x\"{value:04x}\";";)
    }
    if comments {
        concat!(vhdl <= "-- end data_ram.vhd";);
    }
    vhdl
}

fn print_help() {
    println!("{} {}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"),);
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));
    println!("{}", env!("CARGO_PKG_AUTHORS"));
}

fn assembler() -> Result<(), Box<dyn Error>> {
    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            print_help();
            return Ok(());
        }
    };
    let (rom, ram, labels) = parser::parse_file(&path)?;
    if !load::load_cpu(
        &mut Processor::default(),
        Some(&rom),
        Some(&ram),
        Some(labels),
    ) {
        return Err(processor::EmulationError::OutOfRange.into());
    }
    match std::env::args().nth(2) {
        Some(out) => {
            let mut file = std::fs::File::create(format!("{out}.rom.vhd"))?;
            file.write_all(&parse_rom(&rom, false))?;
            let mut file = std::fs::File::create(format!("{out}.ram.vhd"))?;
            file.write_all(&parse_ram(&ram, false))?;
        }
        None => {
            let mut stdout = std::io::stdout();
            stdout.write_all(&parse_rom(&rom, true))?;
            stdout.write_all(&parse_ram(&ram, true))?;
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match assembler() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
