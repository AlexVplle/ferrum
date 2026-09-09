use core::sync::atomic::{AtomicUsize, Ordering};

use super::constants::{
    FIXMAP_BASE, PAGE_MASK, PAGE_SIZE, PAGE_TABLE_ENTRIES, PAGE_TABLE_LEVEL1_SHIFT,
    PAGE_TABLE_LEVEL2_SHIFT, PHYSICAL_TO_VIRTUAL_OFFSET, VIRTUAL_PAGE_NUMBER_MASK,
};
use super::early_paging::EARLY_PAGE_DIRECTORY;
use super::page_table::PageTable;
use super::page_table_entry::PageTableEntry;
use super::page_table_entry_flags::PageTableEntryFlags;

pub const FIXMAP_L2_INDEX: usize =
    (FIXMAP_BASE >> PAGE_TABLE_LEVEL2_SHIFT) & VIRTUAL_PAGE_NUMBER_MASK;
const FIXMAP_L1_INDEX: usize =
    (FIXMAP_BASE >> PAGE_TABLE_LEVEL1_SHIFT) & VIRTUAL_PAGE_NUMBER_MASK;

pub static mut FIXMAP_L1_TABLE: PageTable = PageTable::empty();
static mut FIXMAP_L0_TABLE: PageTable = PageTable::empty();
static FDT_PHYSICAL_PAGE_BASE: AtomicUsize = AtomicUsize::new(0);

pub unsafe fn init() {
    unsafe {
        let non_leaf: PageTableEntryFlags = PageTableEntryFlags::new().valid();

        let l0_physical: usize = (core::ptr::addr_of!(FIXMAP_L0_TABLE) as usize)
            .wrapping_sub(PHYSICAL_TO_VIRTUAL_OFFSET);
        PageTable::write_entry(
            core::ptr::addr_of_mut!(FIXMAP_L1_TABLE),
            FIXMAP_L1_INDEX,
            PageTableEntry::new(l0_physical, non_leaf),
        );

        let l1_physical: usize = (core::ptr::addr_of!(FIXMAP_L1_TABLE) as usize)
            .wrapping_sub(PHYSICAL_TO_VIRTUAL_OFFSET);
        PageTable::write_entry(
            core::ptr::addr_of_mut!(EARLY_PAGE_DIRECTORY),
            FIXMAP_L2_INDEX,
            PageTableEntry::new(l1_physical, non_leaf),
        );
    }
}

pub unsafe fn map_fdt(physical_address: usize) {
    unsafe {
        let base: usize = physical_address & PAGE_MASK;
        FDT_PHYSICAL_PAGE_BASE.store(base, Ordering::Release);

        let leaf: PageTableEntryFlags = PageTableEntryFlags::new().valid().read().accessed();
        let l0_ptr: *mut PageTable = core::ptr::addr_of_mut!(FIXMAP_L0_TABLE);

        for i in 0..PAGE_TABLE_ENTRIES {
            PageTable::write_entry(l0_ptr, i, PageTableEntry::new(base + i * PAGE_SIZE, leaf));
        }

        core::arch::asm!("sfence.vma zero, zero", options(nostack));
    }
}

pub fn fdt_virtual_address(physical_address: usize) -> usize {
    FIXMAP_BASE + physical_address - FDT_PHYSICAL_PAGE_BASE.load(Ordering::Acquire)
}
