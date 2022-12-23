use super::Processor;
use crate::processor::{DisplayRadix, DisplaySigned};
use std::fmt::{Formatter, Result};

impl std::fmt::Display for Processor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.print_registers(f)?;
        self.print_flags(f)?;
        writeln!(f, "Program counter: {}", self.program_counter)?;
        writeln!(f, "Runtime counter: {}", self.runtime_counter)?;
        self.print_ram(f)?;
        self.print_rom(f)?;
        Ok(())
    }
}

impl Processor {
    fn print_registers(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Registers")?;
        for i in 0..self.registers.len() {
            write!(f, "| R{i}: {} ", self.print_value(self.registers[i]))?;
            if i != 0 && (i + 1) % 4 == 0 {
                writeln!(f, "|")?;
            }
        }
        Ok(())
    }

    fn print_flags(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Flags ")?;
        write!(
            f,
            "[ zero: {:#5} ] [ sign: {:#5} ] [ carry: {:#5} ]",
            self.flags.zero, self.flags.sign, self.flags.carry
        )?;
        writeln!(f)?;
        Ok(())
    }

    fn print_ram(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Data memory")?;
        let ram_max = self.ram.len()
            - self
                .ram
                .iter()
                .rev()
                .map_while(|&i| if i == 0 { Some(()) } else { None })
                .count();
        for i in 0..ram_max {
            writeln!(f, "| {:#3} | {}", i, self.print_value(self.ram[i]))?;
        }
        if ram_max < self.ram.len() {
            writeln!(f, "| ··· | {}", self.print_value(0))?;
        }
        Ok(())
    }

    fn print_rom(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Program memory")?;
        for i in 0..self.last_instruction_address() {
            write!(f, "| {:#3} | {}", i, self.rom[i])?;
            if self.program_counter == i {
                write!(f, " <=")?;
            }
            if self.breakpoints[i] {
                writeln!(f, " (*)")?;
            } else {
                writeln!(f)?;
            }
        }
        if self.last_instruction_address() < self.rom.len() {
            write!(f, "| ··· | nop")?;
            if self.program_counter >= self.last_instruction_address() {
                writeln!(f, " <=")?;
            } else {
                writeln!(f)?;
            }
        }
        Ok(())
    }

    fn print_value(&self, value: u16) -> String {
        match &self.radix {
            DisplayRadix::Decimal(signed) => match signed {
                DisplaySigned::Unsigned => format!("{value:5}"),
                DisplaySigned::Signed => format!("{:6}", value as i16),
            },
            DisplayRadix::Hexadecimal => format!("{value:#06x}"),
            DisplayRadix::Binary => format!("{value:#018b}"),
        }
    }
}
