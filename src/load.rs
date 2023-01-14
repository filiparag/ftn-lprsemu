use std::collections::HashMap;

use crate::instructions::Instruction;
use crate::parser::parse_file;
use crate::processor::Processor;

pub fn load_cpu(
    proc: &mut Processor,
    rom: Option<&[Instruction]>,
    ram: Option<&[u16]>,
    labels: Option<HashMap<usize, Vec<String>>>,
) -> bool {
    if let Some(rom) = rom {
        proc.load_rom(rom);
        proc.clear_breakpoints();
        proc.reset();
    }
    if let Some(ram) = ram {
        proc.load_ram(ram);
    }
    if let Some(labels) = labels {
        proc.load_labels(labels);
    }
    match proc.check() {
        Ok(_) => true,
        Err(ins) => {
            eprintln!("Loading error: Instruction '{ins}' is not valid");
            false
        }
    }
}

pub fn load_from_file(proc: &mut Processor, path: &str) -> bool {
    match parse_file(path) {
        Ok((rom, ram, labels)) => load_cpu(proc, Some(&rom), Some(&ram), Some(labels)),
        Err(e) => {
            eprintln!("{e}");
            false
        }
    }
}
