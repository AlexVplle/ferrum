mod constants;
use constants::{CSR_ADDRESS, MODE_MASK, MODE_SV39, PPN_MASK};
use crate::arch::PAGE_SHIFT;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Satp(usize);

impl Satp {
    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn from_bits(bits: usize) -> Self {
        Self(bits)
    }

    pub fn write(self) {
        unsafe {
            core::arch::asm!("csrw {csr}, {0}", in(reg) self.0, csr = const CSR_ADDRESS, options(nostack));
        }
    }

    pub const fn set_sv39(self) -> Self {
        Self::from_bits((self.0 & !MODE_MASK) | MODE_SV39)
    }

    pub const fn with_root_physical_address(self, address: usize) -> Self {
        Self::from_bits((self.0 & !PPN_MASK) | (address >> PAGE_SHIFT))
    }
}
