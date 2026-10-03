#[derive(Clone, Copy, Default)]
pub struct AllocFlags(usize);

impl AllocFlags {
    pub const NONE: Self = Self(0);
    pub const HIGH: Self = Self(0x20);
    pub const HARDER: Self = Self(0x40);
    pub const OUT_OF_MEMORY: Self = Self(0x80);
    pub const HIGHATOMIC: Self = Self(0x200);

    pub fn contains(self, flag: AllocFlags) -> bool {
        self.0 & flag.0 != 0
    }
}

impl core::ops::BitOr for AllocFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
