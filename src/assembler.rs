#![feature(unchecked_math)]

mod asm;
mod instructions;
mod parser;
mod processor;

#[macro_use]
extern crate pest_derive;

fn main() {
    let path;
    match std::env::args().nth(1) {
        Some(p) => path = p,
        None => return,
    };

    #[allow(unused)]
    match parser::parse_file(&path) {
        Ok((rom, ram, labels)) => {
            dbg![rom, ram];
        }
        Err(e) => {
            println!("{e}");
            return;
        }
    }
}
