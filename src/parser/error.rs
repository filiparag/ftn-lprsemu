use super::Rule;
use std::fmt::Display;

#[derive(Debug)]
pub enum ParsingError {
    Filesystem(std::io::Error),
    Pest(Box<pest::error::Error<Rule>>),
    UndefinedLabel(String),
    RedefinedLabel(String),
    NumberConversion(String),
    WrongSection(String),
    UnexpectedToken,
    MissingToken,
    MalformedFile,
}

impl From<std::io::Error> for ParsingError {
    fn from(e: std::io::Error) -> Self {
        println!("{e}");
        Self::Filesystem(e)
    }
}

impl From<pest::error::Error<Rule>> for ParsingError {
    fn from(e: pest::error::Error<Rule>) -> Self {
        Self::Pest(Box::new(e))
    }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parsing error: {}",
            match self {
                Self::Filesystem(e) => e.to_string(),
                Self::Pest(e) => e.to_string(),
                Self::UndefinedLabel(label) => format!("Label '{label}' is not defined"),
                Self::RedefinedLabel(label) => format!("Label '{label}' is defined multiple times"),
                Self::NumberConversion(value) => format!("Unable to convert '{value}' to a number"),
                Self::WrongSection(value) => format!("Found {value} outside its section"),
                e => format!("{e:?}"),
            }
        )
    }
}

impl std::error::Error for ParsingError {}
