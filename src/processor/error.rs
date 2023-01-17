#[derive(Clone, Copy, Debug)]
pub enum EmulationError {
    InvalidLength,
    BinaryParsing,
    InvalidInstruction,
    OutOfRange,
    StackOverflow,
}

impl std::fmt::Display for EmulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for EmulationError {}
