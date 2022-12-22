#![feature(unchecked_math)]

use processor::Processor;
mod asm;
mod error;
mod instructions;
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
    let mut p = Processor::new();

    p.load_ram(asm::DATA_MEMORY);
    // p.load_rom_str(&asm::ROM_BIN).unwrap();
    p.load_rom(asm::ASSEMBLY_CODE);

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
                "pa" | "print auto" => {
                    print_always = !print_always;
                    println!("Auto-print: {print_always}");
                }
                "r" | "run" => {
                    p.run(true);
                    if print_always {
                        println!("{p}")
                    }
                }
                "ra" | "run all" => {
                    p.run(false);
                    if print_always {
                        println!("{p}")
                    }
                }
                "s" | "step" | "" => {
                    p.tick();
                    if print_always {
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
                    println!("  pa | print auto        Toggle state auto-printing");
                    println!("  r  | run               Run until next breakpoint");
                    println!("  ra | run all           Run to the end");
                    println!("  s  | step              Execute one instruction");
                    println!("  b  | breakpoint <line> Set breakpoint on line");
                    println!("  j  | jump <line>       Set program counter to line");
                    println!("  h  | help              Print help");
                }
                _ => println!("Command error"),
            }
        } else {
            println!("Input error");
        }
    }
}
