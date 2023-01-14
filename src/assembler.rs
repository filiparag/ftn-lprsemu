#![feature(unchecked_math)]
#![allow(dead_code)]

use instructions::{Instruction, ToVhdl};
use processor::Processor;
use std::io::Write;

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

fn parse_rom(rom: &[Instruction]) -> Vec<u8> {
    let mut vhdl = Vec::new();
    concat!(vhdl <= "-- instr_rom.vhd";);
    concat!(vhdl <= "architecture Behavioral of instr_rom is";);
    concat!(vhdl <= "begin";);
    for (a, ins) in rom.iter().enumerate() {
        if a == 0 {
            concat!(vhdl <= "    oQ <= ");
        } else {
            concat!(vhdl <= "          ");
        }
        concat!(vhdl <= "\"{}\"  when iA = {a} else", ins.to_vhdl(););
    }
    concat!(vhdl <= "          \"{:015}\";", 0;);
    concat!(vhdl <= "end Behavioral;";);
    vhdl
}

fn parse_ram(ram: &[u16]) -> Vec<u8> {
    let mut vhdl = Vec::new();
    concat!(vhdl <= "-- data_ram.vhd";);
    for (index, value) in ram.iter().enumerate() {
        concat!(vhdl <= "sMEM({index}) <= x\"{value:04x}\";";)
    }
    vhdl
}

fn print_help() {
    println!("{} {}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"),);
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));
    println!("{}", env!("CARGO_PKG_AUTHORS"));
}

fn main() {
    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            print_help();
            return;
        }
    };

    #[allow(unused)]
    match parser::parse_file(&path) {
        Ok((rom, ram, labels)) => {
            if !load::load_cpu(
                &mut Processor::default(),
                Some(&rom),
                Some(&ram),
                Some(labels),
            ) {
                return;
            }
            let mut out = std::io::stdout();
            out.write(&parse_rom(&rom));
            out.write(&parse_ram(&ram));
        }
        Err(e) => {
            println!("{e}");
        }
    }
}
