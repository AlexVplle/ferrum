use ferrum_macros::flag;
use crate::memory_management::physical_address::PhysicalAddress;
use super::constants::{PHYSICAL_PAGE_NUMBER_MASK, PHYSICAL_PAGE_NUMBER_SHIFT};
use super::page_table_entry_flags::PageTableEntryFlags;

#[derive(Copy, Clone)]
pub struct PageTableEntry(usize);

impl PageTableEntry {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn new(physical_address: usize, flags: PageTableEntryFlags) -> Self {
        Self(((physical_address >> 12) << PHYSICAL_PAGE_NUMBER_SHIFT) | flags.bits())
    }

    pub const fn bits(&self) -> usize {
        self.0
    }

    flag!(valid, 0);
    flag!(read, 1);
    flag!(write, 2);
    flag!(execute, 3);
    flag!(user, 4);
    flag!(global, 5);
    flag!(accessed, 6);
    flag!(dirty, 7);

    pub fn is_leaf(&self) -> bool {
        self.is_read() || self.is_execute()
    }

    pub fn physical_address(&self) -> PhysicalAddress {
        PhysicalAddress::new(
            ((self.0 & PHYSICAL_PAGE_NUMBER_MASK) >> PHYSICAL_PAGE_NUMBER_SHIFT) << 12,
        )
    }

    pub fn map(&mut self, physical_address: PhysicalAddress, flags: PageTableEntryFlags) {
        self.0 = ((physical_address.as_usize() >> 12) << PHYSICAL_PAGE_NUMBER_SHIFT) | flags.bits();
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }
}
