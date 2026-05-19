csr!(Scause, usize::MAX, 0x142);

impl Scause {
    pub fn is_interrupt(&self) -> bool {
        self.bits >> 63 != 0
    }

    pub fn code(&self) -> usize {
        self.bits & !(1 << 63)
    }
}
