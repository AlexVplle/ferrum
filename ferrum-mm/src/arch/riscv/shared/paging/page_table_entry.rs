use super::page_table_entry_flags::PageTableEntryFlags;
use crate::arch::{PAGE_SHIFT, PHYSICAL_PAGE_NUMBER_MASK, PHYSICAL_PAGE_NUMBER_SHIFT};
use crate::PhysicalAddress;
use ferrum_macros::flag;

#[derive(Copy, Clone)]
pub struct PageTableEntry(usize);

impl PageTableEntry {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn new(physical_address: PhysicalAddress, flags: PageTableEntryFlags) -> Self {
        let physical_page_number: usize = physical_address.to_page_frame_number();
        let physical_page_number_field: usize = physical_page_number << PHYSICAL_PAGE_NUMBER_SHIFT;
        Self(physical_page_number_field | flags.bits())
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
        let physical_page_number_field: usize = self.0 & PHYSICAL_PAGE_NUMBER_MASK;
        let physical_page_number: usize = physical_page_number_field >> PHYSICAL_PAGE_NUMBER_SHIFT;
        let physical_address: usize = physical_page_number << PAGE_SHIFT;
        PhysicalAddress::new(physical_address)
    }

    pub fn map(&mut self, physical_address: PhysicalAddress, flags: PageTableEntryFlags) {
        let physical_page_number: usize = physical_address.to_page_frame_number();
        let physical_page_number_field: usize = physical_page_number << PHYSICAL_PAGE_NUMBER_SHIFT;
        self.0 = physical_page_number_field | flags.bits();
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }
}
