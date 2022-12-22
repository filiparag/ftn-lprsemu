use super::FlagRegisters;

impl Default for FlagRegisters {
    fn default() -> Self {
        Self {
            zero: false,
            sign: false,
            carry: false,
        }
    }
}

impl FlagRegisters {
    pub fn unset(&mut self) -> &mut Self {
        self.zero = false;
        self.sign = false;
        self.carry = false;
        self
    }
}
