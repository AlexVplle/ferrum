mod constants;

use super::paging::early_paging::EARLY_PAGE_DIRECTORY;
use super::paging::page_table::PageTable;
use super::paging::page_table_entry::PageTableEntry;
use super::paging::page_table_entry_flags::PageTableEntryFlags;
use super::paging::tlb::flush_tlb_all;
use crate::arch::PAGE_SIZE;
use crate::physical_address::PhysicalAddress;
use crate::virtual_address::VirtualAddress;
use constants::FIXMAP_LEVEL1_INDEX;
pub use constants::{FIXMAP_BASE, FIXMAP_LEVEL2_INDEX};
use core::sync::atomic::{AtomicUsize, Ordering};

pub static mut FIXMAP_LEVEL1_TABLE: PageTable = PageTable::empty();
static mut FIXMAP_LEVEL0_TABLE: PageTable = PageTable::empty();
static FDT_PHYSICAL_ADDRESS: AtomicUsize = AtomicUsize::new(0);

pub fn init() {
    let non_leaf: PageTableEntryFlags = PageTableEntryFlags::new().valid();
    let l0_physical: PhysicalAddress =
        VirtualAddress::new(&raw const FIXMAP_LEVEL0_TABLE as usize).to_kernel_physical();
    unsafe {
        FIXMAP_LEVEL1_TABLE[FIXMAP_LEVEL1_INDEX] = PageTableEntry::new(l0_physical, non_leaf)
    };
    let l1_physical: PhysicalAddress =
        VirtualAddress::new(&raw const FIXMAP_LEVEL1_TABLE as usize).to_kernel_physical();
    unsafe {
        EARLY_PAGE_DIRECTORY[FIXMAP_LEVEL2_INDEX] = PageTableEntry::new(l1_physical, non_leaf)
    };
}

pub fn map_fdt(physical_address: PhysicalAddress) {
    FDT_PHYSICAL_ADDRESS.store(physical_address.as_usize(), Ordering::Release);
    let base: PhysicalAddress = physical_address.page_base();
    let within_page_offset: usize = physical_address - base;
    let leaf: PageTableEntryFlags = PageTableEntryFlags::new().valid().read().accessed();

    unsafe {
        FIXMAP_LEVEL0_TABLE[0] = PageTableEntry::new(base, leaf);
        flush_tlb_all();

        let fdt_virtual_start: usize = FIXMAP_BASE + within_page_offset;
        let fdt: fdt::Fdt = fdt::Fdt::from_ptr(fdt_virtual_start as *const u8).unwrap();
        let total_size: usize = fdt.total_size();
        let page_count: usize = (within_page_offset + total_size + PAGE_SIZE - 1) / PAGE_SIZE;

        for i in 1..page_count {
            FIXMAP_LEVEL0_TABLE[i] = PageTableEntry::new(base + i * PAGE_SIZE, leaf);
        }
        if page_count > 1 {
            flush_tlb_all();
        }
    }
}

pub fn fdt_physical_address() -> PhysicalAddress {
    PhysicalAddress::new(FDT_PHYSICAL_ADDRESS.load(Ordering::Acquire))
}

pub fn fdt_virtual_address() -> VirtualAddress {
    let physical_address: PhysicalAddress =
        PhysicalAddress::new(FDT_PHYSICAL_ADDRESS.load(Ordering::Acquire));
    VirtualAddress::new(FIXMAP_BASE + (physical_address - physical_address.page_base()))
}
