use crate::arch::PAGE_SIZE;
use crate::memory_management::physical_address::PhysicalAddress;

pub trait PhysicalAllocator {
    fn alloc(&mut self, size: usize, align: usize) -> Option<PhysicalAddress>;
    fn free(&mut self, address: PhysicalAddress, size: usize);

    fn alloc_page(&mut self) -> Option<PhysicalAddress> {
        self.alloc(PAGE_SIZE, PAGE_SIZE)
    }

    fn free_page(&mut self, address: PhysicalAddress) {
        self.free(address, PAGE_SIZE);
    }
}
