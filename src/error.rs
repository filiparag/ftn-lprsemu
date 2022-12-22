#[derive(Clone, Copy, Debug)]
pub enum EmulationError {
    InvalidLength,
    BinaryParsing,
    InvalidInstruction,
}
