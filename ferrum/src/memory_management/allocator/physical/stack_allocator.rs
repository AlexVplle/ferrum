use crate::memory_management::physical_address::PhysicalAddress;
use crate::memory_management::virtual_address::VirtualAddress;
use super::allocator::PhysicalAllocator;

pub struct StackAllocator(usize);

impl StackAllocator {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn push(&mut self, address: PhysicalAddress) {
        unsafe {
            *(address.to_virtual().as_usize() as *mut usize) = self.0;
        }
        self.0 = address.as_usize();
    }
}

impl PhysicalAllocator for StackAllocator {
    fn alloc(&mut self, _size: usize, _align: usize) -> Option<PhysicalAddress> {
        if self.0 == 0 {
            return None;
        }
        let physical_address: PhysicalAddress = PhysicalAddress::new(self.0);
        let virtual_address: VirtualAddress = physical_address.to_virtual();
        self.0 = unsafe { *(virtual_address.as_usize() as *const usize) };
        Some(physical_address)
    }

    fn free(&mut self, address: PhysicalAddress, _size: usize) {
        self.push(address);
    }
}
