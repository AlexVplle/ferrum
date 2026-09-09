mod constants;

use super::paging::early_paging::EARLY_PAGE_DIRECTORY;
use super::paging::page_table::PageTable;
use super::paging::page_table_entry::PageTableEntry;
use super::paging::page_table_entry_flags::PageTableEntryFlags;
use crate::arch::{PAGE_MASK, PAGE_SIZE, PAGE_TABLE_ENTRIES, PHYSICAL_TO_VIRTUAL_OFFSET};
use constants::FIXMAP_LEVEL1_INDEX;
pub use constants::{FIXMAP_BASE, FIXMAP_LEVEL2_INDEX};
use core::sync::atomic::{AtomicUsize, Ordering};

pub static mut FIXMAP_LEVEL1_TABLE: PageTable = PageTable::empty();
static mut FIXMAP_L0_TABLE: PageTable = PageTable::empty();
static FDT_VIRTUAL_ADDRESS: AtomicUsize = AtomicUsize::new(0);

pub fn init() {
    let non_leaf: PageTableEntryFlags = PageTableEntryFlags::new().valid();
    let l0_physical: usize =
        (&raw const FIXMAP_L0_TABLE as usize).wrapping_sub(PHYSICAL_TO_VIRTUAL_OFFSET);
    unsafe {
        FIXMAP_LEVEL1_TABLE[FIXMAP_LEVEL1_INDEX] = PageTableEntry::new(l0_physical, non_leaf)
    };
    let l1_physical: usize =
        (&raw const FIXMAP_LEVEL1_TABLE as usize).wrapping_sub(PHYSICAL_TO_VIRTUAL_OFFSET);
    unsafe {
        EARLY_PAGE_DIRECTORY[FIXMAP_LEVEL2_INDEX] = PageTableEntry::new(l1_physical, non_leaf)
    };
}

pub unsafe fn map_fdt(physical_address: usize) {
    let base: usize = physical_address & PAGE_MASK;
    FDT_VIRTUAL_ADDRESS.store(FIXMAP_BASE + (physical_address - base), Ordering::Release);
    let leaf: PageTableEntryFlags = PageTableEntryFlags::new().valid().read().accessed();
    unsafe {
        for i in 0..PAGE_TABLE_ENTRIES {
            FIXMAP_L0_TABLE[i] = PageTableEntry::new(base + i * PAGE_SIZE, leaf);
        }
        core::arch::asm!("sfence.vma zero, zero", options(nostack));
    }
}

pub fn fdt_virtual_address() -> usize {
    FDT_VIRTUAL_ADDRESS.load(Ordering::Acquire)
}
