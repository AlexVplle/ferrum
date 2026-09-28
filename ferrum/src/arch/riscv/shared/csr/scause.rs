csr!(Scause, usize::MAX, 0x142);

impl Scause {
    pub fn is_interrupt(&self) -> bool {
        self.bits() >> (usize::BITS - 1) != 0
    }

    pub fn code(&self) -> usize {
        self.bits() & !(1usize << (usize::BITS - 1))
    }
}
