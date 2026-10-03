use ferrum_macros::AddressFunctions;
use super::virtual_address::VirtualAddress;

#[derive(Clone, Copy, AddressFunctions)]
#[repr(transparent)]
pub struct PhysicalAddress(usize);

impl PhysicalAddress {
    pub fn to_virtual(self) -> VirtualAddress {
        VirtualAddress::new(self.0.wrapping_add(crate::arch::page_offset()))
    }

    pub fn to_kernel_virtual(self) -> VirtualAddress {
        VirtualAddress::new(self.0.wrapping_add(crate::arch::physical_to_virtual_offset()))
    }

    pub const fn to_page_frame_number(self) -> usize {
        self.0 >> crate::arch::PAGE_SHIFT
    }

    pub const fn page_base(self) -> Self {
        Self::new(self.0 & crate::arch::PAGE_MASK)
    }


}
