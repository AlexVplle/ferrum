use ferrum_macros::AddressFunctions;
use super::physical_address::PhysicalAddress;

#[derive(Clone, Copy, AddressFunctions)]
#[repr(transparent)]
pub struct VirtualAddress(usize);

impl VirtualAddress {
    pub fn to_physical(self) -> PhysicalAddress {
        PhysicalAddress::new(self.as_usize() - crate::arch::PAGE_OFFSET)
    }
}
