use super::FlagRegisters;

impl FlagRegisters {
    pub fn unset(&mut self) -> &mut Self {
        self.zero = false;
        self.sign = false;
        self.carry = false;
        self
    }
}
