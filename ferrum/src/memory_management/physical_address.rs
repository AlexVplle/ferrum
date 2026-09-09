use ferrum_macros::AddressFunctions;
use super::virtual_address::VirtualAddress;

#[derive(Clone, Copy, AddressFunctions)]
#[repr(transparent)]
pub struct PhysicalAddress(usize);

impl PhysicalAddress {
    pub fn to_virtual(self) -> VirtualAddress {
        VirtualAddress::new(self.as_usize() + crate::arch::PAGE_OFFSET)
    }
}
