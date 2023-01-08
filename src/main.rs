#![feature(unchecked_math)]

#[macro_use]
extern crate pest_derive;

mod asm;
mod instructions;
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

fn main() {
    use processor::Processor;
    let mut p = Processor::new();
    p.load_rom(asm::ROM_ASM);
    p.load_ram(asm::DATA_MEMORY);
    let mut print_always: bool = true;
    println!("{p}");
    loop {
        if let Some(input) = prompt(">>") {
            if input.is_empty() {
                continue;
            }
            if print_always {
                print!("{}[2J", 27 as char);
            }
            match input[0].as_str() {
                "p" | "print" => println!("{p}"),
                "pa" | "print-auto" => {
                    print_always = !print_always;
                    println!("Auto-print: {print_always}");
                }
                "la" | "load-asm" => {
                    p.load_rom(asm::ROM_ASM);
                    p.load_ram(asm::DATA_MEMORY);
                    p.reset();
                    if print_always {
                        println!("{p}")
                    }
                }
                "lb" | "load-bin" => {
                    if let Err(e) = p.load_rom_str(&asm::ROM_BIN) {
                        println!("Loading error: {e:?}");
                    } else {
                        p.load_ram(asm::DATA_MEMORY);
                        p.reset();
                        if print_always {
                            println!("{p}")
                        }
                    }
                }
                "lf" | "load-file" => {
                    if input.len() != 2 {
                        println!("Argument error");
                        continue;
                    }
                    match parser::parse_file(input[1].as_str()) {
                        Ok((rom, ram, labels)) => {
                            p.load_rom(&rom);
                            p.load_ram(&ram);
                            p.load_labels(labels);
                            if print_always {
                                println!("{p}")
                            }
                        }
                        Err(e) => println!("Loading error: {e:?}"),
                    }
                }
                "d" | "radix" => {
                    if input.len() != 2 {
                        println!("Argument error");
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
                    if print_always {
                        println!("{p}")
                    }
                }
                "r" | "run" => {
                    if let Err(e) = p.run(true) {
                        println!("Emulation error: {e:?}")
                    } else if print_always {
                        println!("{p}")
                    }
                }
                "ra" | "run-all" => {
                    if let Err(e) = p.run(false) {
                        println!("Emulation error: {e:?}")
                    } else if print_always {
                        println!("{p}")
                    }
                }
                "s" | "step" | "" => {
                    if let Err(e) = p.tick() {
                        println!("Emulation error: {e:?}")
                    } else if print_always {
                        println!("{p}")
                    }
                }
                "b" | "breakpoint" => {
                    if input.len() != 2 {
                        println!("Argument error");
                        continue;
                    }
                    let line: usize = input[1].parse().expect("Parsing error");
                    p.toggle_breakpoint(line);
                    if print_always {
                        println!("{p}")
                    }
                }
                "bc" | "breakpoint-clear" => {
                    p.clear_breakpoints();
                    if print_always {
                        println!("{p}")
                    }
                }
                "j" | "jump" => {
                    if input.len() != 2 {
                        println!("Argument error");
                        continue;
                    }
                    let line: usize = input[1].parse().expect("Parsing error");
                    p.program_counter_jump(line);
                    if print_always {
                        println!("{p}")
                    }
                }
                "x" | "reset" => {
                    p.load_ram(asm::DATA_MEMORY);
                    p.load_rom(asm::ROM_ASM);
                    p.reset();
                    if print_always {
                        println!("{p}")
                    }
                }
                "e" | "benchmark" => {
                    p.load_rom(asm::BENCHMARK);
                    p.reset();
                    let stopwatch = std::time::Instant::now();
                    match p.run(false) {
                        Ok(ticks) => println!(
                            "Emulation speed: {:.2} MHz",
                            ticks as f64 / stopwatch.elapsed().as_secs_f64() / 1e6
                        ),
                        Err(e) => println!("Emulation error: {e:?}"),
                    }
                }
                "h" | "help" => {
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
                    println!("  pa | print-auto        Toggle state auto-printing");
                    println!("  la | load-asm          Load program from ROM_ASM macro");
                    println!("  lb | load-bin          Load program from ROM_BIN strings");
                    println!("  lf | load-file <path>  Load program from assembly file");
                    println!("  d  | radix <u/s/x/b>   Toggle decimal display form");
                    println!("  r  | run               Run until next breakpoint");
                    println!("  ra | run-all           Run to the end");
                    println!("  s  | step              Execute one instruction");
                    println!("  b  | breakpoint <line> Set breakpoint on line");
                    println!("  bc | breakpoint-clear  Remove all breakpoints");
                    println!("  j  | jump <line>       Set program counter to line");
                    println!("  x  | reset             Reset processor");
                    println!("  e  | benchmark         Emulation speed benchmark");
                    println!("  h  | help              Print help");
                }
                _ => println!("Command error"),
            }
        } else {
            println!("Input error");
        }
    }
}
