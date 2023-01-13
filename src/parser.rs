use pest::iterators::Pair;
use pest::Parser;

use crate::instructions::Instruction;
use crate::processor::{RAM_SIZE, ROM_SIZE};

use std::collections::HashMap;

mod data;
mod text;

use data::parse_data;
use text::{parse_instructions, Labels, RawInstruction, RawInstructions};

#[derive(Parser)]
#[grammar = "src/parser/isa.pest"]
pub struct AsmFileParser;

pub type AsmFileData = (Vec<Instruction>, Vec<u16>, HashMap<usize, Vec<String>>);

struct AsmFile<'a> {
    data: Vec<u16>,
    instructions: RawInstructions<'a>,
    labels: Labels<'a>,
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
            Rule::data => {
                if let Some(ProgramSection::Data) = current_section {
                    asmfile.data.push(parse_data(line)?);
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
