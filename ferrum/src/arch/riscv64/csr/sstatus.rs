csr!(Sstatus, 0x8000_0000_000d_e762, 0x100);

impl Sstatus {
    pub fn spp(&self) -> bool {
        self.bits & (1 << 8) != 0
    }

    pub fn spie(&self) -> bool {
        self.bits & (1 << 5) != 0
    }

    pub fn sie(&self) -> bool {
        self.bits & (1 << 1) != 0
    }
}
