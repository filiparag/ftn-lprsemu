#![feature(unchecked_shifts)]
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

fn parse_rom(rom: &[Instruction]) -> Result<Vec<u8>, Box<dyn Error>> {
    let rom_prefix = include_bytes!("../data/rom_prefix.vhd");
    let rom_suffix = include_bytes!("../data/rom_suffix.vhd");
    let mut vhdl = Vec::from(rom_prefix);
    rom.iter().enumerate().try_for_each(|(addr, instr)| {
        writeln!(vhdl, "\t\t\"{}\"  when iA = {addr} else", instr.to_vhdl())
    })?;
    vhdl.extend_from_slice(rom_suffix);
    Ok(vhdl)
}

fn parse_ram(ram: &[u16]) -> Result<Vec<u8>, Box<dyn Error>> {
    let ram_prefix = include_bytes!("../data/ram_prefix.vhd");
    let ram_suffix = include_bytes!("../data/ram_suffix.vhd");
    let mut vhdl = Vec::from(ram_prefix);
    ram.iter()
        .enumerate()
        .try_for_each(|(index, value)| writeln!(vhdl, "\tsMEM({index}) <= x\"{value:04x}\";"))?;
    vhdl.extend_from_slice(ram_suffix);
    Ok(vhdl)
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
            let mut file = std::fs::File::create(format!("{out}instr_rom.vhd"))?;
            file.write_all(&parse_rom(&rom)?)?;
            let mut file = std::fs::File::create(format!("{out}data_ram.vhd"))?;
            file.write_all(&parse_ram(&ram)?)?;
        }
        None => {
            let mut stdout = std::io::stdout();
            stdout.write_all("-- begin instr_rom.vhd\n".as_bytes())?;
            stdout.write_all(&parse_rom(&rom)?)?;
            stdout.write_all("-- end instr_rom.vhd\n\n-- begin data_ram.vhd\n".as_bytes())?;
            stdout.write_all(&parse_ram(&ram)?)?;
            stdout.write_all("-- end data_ram.vhd\n".as_bytes())?;
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
