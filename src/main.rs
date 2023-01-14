#![feature(unchecked_math)]

use processor::Processor;

#[macro_use]
extern crate pest_derive;

mod asm;
mod instructions;
mod load;
mod parser;
mod processor;

fn prompt(separator: &str) -> Option<Vec<String>> {
    use std::io::Write;
    let mut line = String::new();
    print!("{} {} ", env!("CARGO_PKG_NAME"), separator);
    std::io::stdout().flush().unwrap();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => Some(line.trim().split(' ').map(str::to_string).collect()),
        Err(_) => None,
    }
}

fn print_help() {
    println!(
        "{} {} - {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION")
    );
    println!("{}", env!("CARGO_PKG_AUTHORS"));
    println!();
    println!("Usage:");
    println!("  p  | print             Print current state");
    println!("  l  | load-file <path>  Load program from assembly file");
    println!("  d  | radix <u/s/x/b>   Toggle decimal display form");
    println!("  r  | run               Run until next breakpoint");
    println!("  ra | run-all           Run to the end");
    println!("  s  | step              Execute one instruction");
    println!("  u  | undo              Undo last instruction");
    println!("  b  | breakpoint <line> Toggle breakpoint on line");
    println!("  bc | breakpoint-clear  Remove all breakpoints");
    println!("  j  | jump <line>       Set program counter to line");
    println!("  x  | reset             Reset processor");
    println!("  e  | benchmark         Emulation speed benchmark");
    println!("  h  | help              Print help");
}

fn main() {
    let mut p = Processor::default();

    match std::env::args().nth(1) {
        Some(path) => {
            if !load::load_from_file(&mut p, &path) {
                return;
            }
        }
        None => {
            if !load::load_cpu(&mut p, Some(asm::ROM_ASM), Some(asm::DATA_MEMORY), None) {
                return;
            }
        }
    };

    println!("{p}");
    loop {
        if let Some(input) = prompt(">>") {
            if input.is_empty() {
                continue;
            }
            _ = clearscreen::clear();
            match input[0].as_str() {
                "p" | "print" => println!("{p}"),
                "l" | "load" => {
                    if input.len() != 2 {
                        eprintln!("Argument error");
                        continue;
                    }
                    if load::load_from_file(&mut p, input[1].as_str()) {
                        println!("{p}");
                    }
                }
                "d" | "radix" => {
                    if input.len() != 2 {
                        eprintln!("Argument error");
                        continue;
                    }
                    use processor::{DisplayRadix, DisplaySigned};
                    match input[1].as_str() {
                        "u" => p.set_radix(DisplayRadix::Decimal(DisplaySigned::Unsigned)),
                        "s" => p.set_radix(DisplayRadix::Decimal(DisplaySigned::Signed)),
                        "x" => p.set_radix(DisplayRadix::Hexadecimal),
                        "b" => p.set_radix(DisplayRadix::Binary),
                        _ => println!("Argument error"),
                    }
                    println!("{p}");
                }
                "r" | "run" => {
                    if let Err(e) = p.run(true, None) {
                        eprintln!("Emulation error: {e:?}")
                    } else {
                        println!("{p}");
                    }
                }
                "ra" | "run-all" => {
                    if let Err(e) = p.run(false, None) {
                        eprintln!("Emulation error: {e:?}")
                    } else {
                        println!("{p}");
                    }
                }
                "s" | "step" | "" => {
                    if let Err(e) = p.tick() {
                        eprintln!("Emulation error: {e:?}")
                    } else {
                        println!("{p}");
                    }
                }
                "u" | "undo" => {
                    if let Err(e) = p.run(false, Some(-1)) {
                        eprintln!("Emulation error: {e:?}")
                    } else {
                        println!("{p}");
                    }
                }
                "b" | "breakpoint" => {
                    if input.len() != 2 {
                        eprintln!("Argument error");
                        continue;
                    }
                    let line: usize = input[1].parse().expect("Line parsing error");
                    p.toggle_breakpoint(line);
                    println!("{p}");
                }
                "bc" | "breakpoint-clear" => {
                    p.clear_breakpoints();
                    println!("{p}");
                }
                "j" | "jump" => {
                    if input.len() != 2 {
                        eprintln!("Argument error");
                        continue;
                    }
                    let line: usize = input[1].parse().expect("Line parsing error");
                    p.program_counter_jump(line);
                    println!("{p}");
                }
                "x" | "reset" => {
                    p.reset();
                    println!("{p}");
                }
                "e" | "benchmark" => {
                    p.load_rom(asm::BENCHMARK);
                    p.reset();
                    let stopwatch = std::time::Instant::now();
                    match p.run(false, None) {
                        Ok(ticks) => println!(
                            "Emulation speed: {:.2} MIPS",
                            ticks as f64 / stopwatch.elapsed().as_secs_f64() / 1e6
                        ),
                        Err(e) => eprintln!("Emulation error: {e:?}"),
                    }
                }
                "h" | "help" => print_help(),
                _ => eprintln!("Command error"),
            }
        } else {
            eprintln!("Input error");
        }
    }
}
