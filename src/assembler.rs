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
    let rom_prefix = r#"

-------------------------------------------------------
-- Logicko projektovanje racunarskih sistema 1
-- 2011/2012, 2023
--
-- Instruction ROM
--
-- authors:
-- Ivan Kastelan (ivan.kastelan@rt-rk.com)
-- Milos Subotic (milos.subotic@uns.ac.rs)
-------------------------------------------------------

library ieee;
use ieee.std_logic_1164.all;
use ieee.std_logic_unsigned.all;

entity instr_rom is
	port(
		iA : in  std_logic_vector(7 downto 0);
		oQ : out std_logic_vector(14 downto 0)
);
end instr_rom;

architecture arch of instr_rom is
begin
	oQ <= 
------------------------------------------------------------------
"#;

    let rom_suffix = r#"
------------------------------------------------------------------
		"000000000000000";
end architecture;
"#;

    let mut vhdl = Vec::new();
    write!(vhdl, "{}", rom_prefix);
    for (a, ins) in rom.iter().enumerate() {
        concat!(vhdl <= "		\"{}\"  when iA = {a} else", ins.to_vhdl(););
    }
    write!(vhdl, "{}", rom_suffix);
    vhdl
}



fn parse_ram(ram: &[u16], comments: bool) -> Vec<u8> {
    let ram_prefix = r#"

-------------------------------------------------------
-- Logicko projektovanje racunarskih sistema 1
-- 2011/2012, 2023
--
-- Data RAM
--
-- authors:
-- Ivan Kastelan (ivan.kastelan@rt-rk.com)
-- Milos Subotic (milos.subotic@uns.ac.rs)
-------------------------------------------------------

library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity data_ram is
	port(
		iCLK  : in  std_logic;
		inRST : in  std_logic;
		iA    : in  std_logic_vector(7 downto 0);
		iD    : in  std_logic_vector(15 downto 0);
		iWE   : in  std_logic;
		oQ    : out std_logic_vector(15 downto 0)
	);
end data_ram;

architecture arch of data_ram is

	type tMEM is array(0 to 255) of std_logic_vector(15 downto 0);
	signal rMEM : tMEM;
	signal sMEM : tMEM := (others => x"0000");

begin

	process(iCLK, inRST)begin
		if inRST = '0' then
			for i in 0 to 255 loop
				rMEM(i) <= sMEM(i); 
			end loop;
		elsif rising_edge(iCLK) then
			if iWE = '1' then
				rMEM(to_integer(unsigned(iA))) <= iD;
			end if;
		end if;
	end process;

------------------------------------------------------------------
"#;

    let ram_suffix = r#"
------------------------------------------------------------------
	
	oQ <= rMEM(to_integer(unsigned(iA)));

end architecture;
"#;
    let mut vhdl = Vec::new();
    write!(vhdl, "{}", ram_prefix);
    for (index, value) in ram.iter().enumerate() {
        concat!(vhdl <= "	sMEM({index}) <= x\"{value:04x}\";";)
    }
    write!(vhdl, "{}", ram_suffix);
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
            let mut file = std::fs::File::create(format!("{out}instr_rom.vhd"))?;
            file.write_all(&parse_rom(&rom, false))?;
            let mut file = std::fs::File::create(format!("{out}data_ram.vhd"))?;
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
