csr!(Satp, usize::MAX, 0x180);

impl Satp {
    pub fn set_sv39(self) -> Self {
        Self::from_bits((self.0 & !(0xF << 60)) | (8 << 60))
    }

    pub fn with_root_physical_address(self, address: usize) -> Self {
        Self::from_bits((self.0 & !0x0fff_ffff_ffff) | (address >> 12))
    }

    pub fn mode(&self) -> usize {
        self.bits() >> 60
    }

    pub fn address_space_id(&self) -> usize {
        (self.bits() >> 44) & 0xffff
    }

    pub fn physical_page_number(&self) -> usize {
        self.bits() & 0x0fff_ffff_ffff
    }

    pub fn root_physical_address(&self) -> usize {
        self.physical_page_number() << 12
    }
}
