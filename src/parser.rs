use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;

use crate::instructions::Instruction;
use crate::op;
use crate::processor::{RAM_SIZE, ROM_SIZE};

#[derive(Parser)]
#[grammar = "src/parser/isa.pest"]
pub struct AsmFileParser;

type RawInstructions<'a> = Vec<RawInstruction<'a>>;
type Labels<'a> = HashMap<&'a str, usize>;
pub type AsmFileData = (Vec<Instruction>, Vec<u16>, HashMap<usize, Vec<String>>);

struct AsmFile<'a> {
    data: Vec<u16>,
    instructions: Vec<RawInstruction<'a>>,
    labels: HashMap<&'a str, usize>,
}

#[derive(Debug)]
enum RawInstruction<'a> {
    Label {
        op: &'a str,
        label: &'a str,
    },
    Reg2 {
        op: &'a str,
        z: &'a str,
        x: &'a str,
    },
    Reg3 {
        op: &'a str,
        z: &'a str,
        x: &'a str,
        y: &'a str,
    },
}

fn parse_instruction_l(pair: Pair<'_, Rule>) -> Result<RawInstruction, ()> {
    let mut pairs;
    if let Rule::instr_l = pair.as_rule() {
        pairs = pair.into_inner();
    } else {
        return Err(());
    }
    match (pairs.next(), pairs.next()) {
        (Some(op), Some(label)) => Ok(RawInstruction::Label {
            op: op.as_str(),
            label: label.as_str(),
        }),
        _ => Err(()),
    }
}

fn parse_instruction_2r(pair: Pair<'_, Rule>) -> Result<RawInstruction, ()> {
    let mut pairs;
    if let Rule::instr_2r = pair.as_rule() {
        pairs = pair.into_inner();
    } else {
        return Err(());
    }
    let op = pairs.next();
    if let (Some(r1), Some(r2)) = (
        pairs.next().into_iter().next(),
        pairs.next().into_iter().next(),
    ) {
        match (op, r1.into_inner().next(), r2.into_inner().next()) {
            (Some(op), Some(r1), Some(r2)) => Ok(RawInstruction::Reg2 {
                op: op.as_str(),
                z: r1.as_str(),
                x: r2.as_str(),
            }),
            _ => Err(()),
        }
    } else {
        Err(())
    }
}

fn parse_instruction_3r(pair: Pair<'_, Rule>) -> Result<RawInstruction, ()> {
    let mut pairs;
    if let Rule::instr_3r = pair.as_rule() {
        pairs = pair.into_inner();
    } else {
        return Err(());
    }
    let op = pairs.next();
    if let (Some(r1), Some(r2), Some(r3)) = (
        pairs.next().into_iter().next(),
        pairs.next().into_iter().next(),
        pairs.next().into_iter().next(),
    ) {
        match (
            op,
            r1.into_inner().next(),
            r2.into_inner().next(),
            r3.into_inner().next(),
        ) {
            (Some(op), Some(r1), Some(r2), Some(r3)) => Ok(RawInstruction::Reg3 {
                op: op.as_str(),
                z: r1.as_str(),
                x: r2.as_str(),
                y: r3.as_str(),
            }),
            _ => Err(()),
        }
    } else {
        Err(())
    }
}

impl<'a> TryFrom<Pair<'a, Rule>> for RawInstruction<'a> {
    type Error = ();
    fn try_from(value: Pair<'a, Rule>) -> Result<Self, Self::Error> {
        if let Rule::instruction = value.as_rule() {
            if let Some(value) = value.into_inner().next() {
                match value.as_rule() {
                    Rule::instr_l => parse_instruction_l(value),
                    Rule::instr_2r => parse_instruction_2r(value),
                    Rule::instr_3r => parse_instruction_3r(value),
                    _ => Err(()),
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

fn parse_instructions<'a>(
    instructions: &RawInstructions<'a>,
    labels: &Labels<'a>,
) -> Result<Vec<Instruction>, ParsingError> {
    let mut processed: Vec<Instruction> = Vec::with_capacity(instructions.len());
    for ins in instructions {
        match ins {
            RawInstruction::Label { op, label } => {
                let line = match labels.get(label) {
                    Some(l) => *l as u16,
                    None => return Err(ParsingError::UndefinedLabel),
                };
                match *op {
                    "jmp" => processed.push(op![jmp line]),
                    "jmpz" => processed.push(op![jmpz line]),
                    "jmps" => processed.push(op![jmps line]),
                    "jmpc" => processed.push(op![jmpc line]),
                    "jmpnz" => processed.push(op![jmpnz line]),
                    "jmpns" => processed.push(op![jmpns line]),
                    "jmpnc" => processed.push(op![jmpnc line]),
                    _ => return Err(ParsingError::InvalidInstruction),
                }
            }
            RawInstruction::Reg2 { op, z, x } => match (*op, z.parse::<u8>(), x.parse::<u8>()) {
                ("mov", Ok(z), Ok(x)) => processed.push(op![mov z, x]),
                ("inc", Ok(z), Ok(x)) => processed.push(op![inc z, x]),
                ("dec", Ok(z), Ok(x)) => processed.push(op![dec z, x]),
                ("shl", Ok(z), Ok(x)) => processed.push(op![shl z, x]),
                ("shr", Ok(z), Ok(x)) => processed.push(op![shr z, x]),
                ("ashl", Ok(z), Ok(x)) => processed.push(op![ashl z, x]),
                ("ashr", Ok(z), Ok(x)) => processed.push(op![ashr z, x]),
                ("ld", Ok(z), Ok(y)) => processed.push(op![ld z, y]),
                ("st", Ok(z), Ok(x)) => processed.push(op![st z, x]),
                _ => return Err(ParsingError::InvalidInstruction),
            },
            RawInstruction::Reg3 { op, z, x, y } => {
                match (*op, z.parse::<u8>(), x.parse::<u8>(), y.parse::<u8>()) {
                    ("add", Ok(z), Ok(x), Ok(y)) => processed.push(op![add z, x, y]),
                    ("sub", Ok(z), Ok(x), Ok(y)) => processed.push(op![sub z, x, y]),
                    ("and", Ok(z), Ok(x), Ok(y)) => processed.push(op![and z, x, y]),
                    ("or", Ok(z), Ok(x), Ok(y)) => processed.push(op![or z, x, y]),
                    _ => return Err(ParsingError::InvalidInstruction),
                }
            }
        }
    }
    Ok(processed)
}

fn parse_radix(pair: Pair<'_, Rule>) -> Result<u16, ParsingError> {
    let data = pair.as_span().as_str();
    if data.len() < 3 {
        return Err(ParsingError::MalformedFile);
    }
    match &data[0..=1] {
        "0x" => {
            if let Ok(result) = u16::from_str_radix(data.trim_start_matches("0x"), 16) {
                Ok(result)
            } else {
                Err(ParsingError::MalformedFile)
            }
        }
        "0b" => {
            if let Ok(result) = u16::from_str_radix(data.trim_start_matches("0b"), 2) {
                Ok(result)
            } else {
                Err(ParsingError::MalformedFile)
            }
        }
        _ => Err(ParsingError::UnexpectedToken),
    }
}

#[derive(Debug)]
pub enum ParsingError {
    Filesystem(std::io::Error),
    Pest(pest::error::Error<Rule>),
    InvalidInstruction,
    UndefinedLabel,
    MalformedFile,
    UnexpectedToken,
}

impl From<std::io::Error> for ParsingError {
    fn from(e: std::io::Error) -> Self {
        Self::Filesystem(e)
    }
}

impl From<pest::error::Error<Rule>> for ParsingError {
    fn from(e: pest::error::Error<Rule>) -> Self {
        Self::Pest(e)
    }
}

#[derive(Debug)]
enum ProgramSection {
    Data,
    Text,
    Unknown,
}

impl From<&str> for ProgramSection {
    fn from(value: &str) -> Self {
        match value {
            "data" => Self::Data,
            "text" => Self::Text,
            _ => Self::Unknown,
        }
    }
}

pub fn parse_file(path: &str) -> Result<AsmFileData, ParsingError> {
    let text = std::fs::read_to_string(path)?;
    let file = AsmFileParser::parse(Rule::file, &text)?.next();
    let pairs;
    if let Some(p) = file {
        pairs = p;
    } else {
        return Err(ParsingError::MalformedFile);
    }
    let mut asmfile = AsmFile {
        data: Vec::with_capacity(RAM_SIZE),
        instructions: Vec::with_capacity(ROM_SIZE),
        labels: HashMap::with_capacity(ROM_SIZE),
    };
    let mut current_section: Option<ProgramSection> = None;
    for line in pairs.into_inner() {
        match line.as_rule() {
            Rule::section => {
                if let Some(section) = line.into_inner().next() {
                    current_section = Some(section.as_str().into());
                } else {
                    return Err(ParsingError::MalformedFile);
                }
            }
            Rule::label => {
                if let Some(ProgramSection::Text) = current_section {
                    if let Some(label) = line.into_inner().next() {
                        asmfile
                            .labels
                            .insert(label.as_str(), asmfile.instructions.len());
                    } else {
                        return Err(ParsingError::MalformedFile);
                    }
                } else {
                    return Err(ParsingError::UnexpectedToken);
                }
            }
            Rule::instruction => {
                if let Ok(i) = RawInstruction::try_from(line) {
                    asmfile.instructions.push(i);
                }
            }
            Rule::number => {
                if let Some(ProgramSection::Data) = current_section {
                    if let Ok(data) = line.as_span().as_str().parse::<u16>() {
                        asmfile.data.push(data);
                    } else {
                        return Err(ParsingError::MalformedFile);
                    }
                } else {
                    return Err(ParsingError::UnexpectedToken);
                }
            }
            Rule::radix => {
                if let Some(ProgramSection::Data) = current_section {
                    asmfile.data.push(parse_radix(line)?);
                } else {
                    return Err(ParsingError::UnexpectedToken);
                }
            }
            Rule::EOI => (),
            _ => {}
        }
    }
    let instructions = parse_instructions(&asmfile.instructions, &asmfile.labels)?;
    let mut labels: HashMap<usize, Vec<String>> = HashMap::with_capacity(asmfile.labels.len());
    for (k, v) in &asmfile.labels {
        if let Some(bucket) = labels.get_mut(v) {
            bucket.push((*k).to_owned());
        } else {
            labels.insert(*v, vec![(*k).to_owned()]);
        }
    }
    Ok((instructions, asmfile.data, labels))
}
