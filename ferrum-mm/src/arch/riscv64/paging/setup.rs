use super::super::fixmap::FIXMAP_LEVEL2_INDEX;
use super::constants::KERNEL_LEVEL2_INDEX;
use super::early_paging::EARLY_PAGE_DIRECTORY;
use super::page_table::PageTable;
use super::page_table_entry::PageTableEntry;
use super::page_table_entry_flags::PageTableEntryFlags;
use super::satp::Satp;
use crate::arch::{GIGA_PAGE_SIZE, PAGE_TABLE_LEVEL2_SHIFT, VIRTUAL_PAGE_NUMBER_MASK};
use crate::memory_block::{MemoryBlockRegion, MEMORY_BLOCK};
use crate::physical_address::PhysicalAddress;
use crate::virtual_address::VirtualAddress;

#[unsafe(no_mangle)]
static mut swapper_page_directory: PageTable = PageTable::empty();

pub fn setup_direct_map() {
    let regions: &[MemoryBlockRegion] = unsafe { (*(&raw const MEMORY_BLOCK)).memory_regions() };
    unsafe {
        swapper_page_directory[KERNEL_LEVEL2_INDEX] = EARLY_PAGE_DIRECTORY[KERNEL_LEVEL2_INDEX]
    };
    unsafe {
        swapper_page_directory[FIXMAP_LEVEL2_INDEX] = EARLY_PAGE_DIRECTORY[FIXMAP_LEVEL2_INDEX]
    };

    let flags: PageTableEntryFlags = PageTableEntryFlags::new()
        .valid()
        .read()
        .write()
        .execute()
        .global()
        .accessed()
        .dirty();

    for region in regions {
        let start: PhysicalAddress = region.base.giga_page_base();
        let end: PhysicalAddress =
            (region.base + region.size + GIGA_PAGE_SIZE - 1).giga_page_base();

        let mut gigapage_physical_address: PhysicalAddress = start;
        while gigapage_physical_address.as_usize() < end.as_usize() {
            let direct_map_virtual_address: VirtualAddress = gigapage_physical_address.to_virtual();
            let level2_index: usize = (direct_map_virtual_address.as_usize()
                >> PAGE_TABLE_LEVEL2_SHIFT)
                & VIRTUAL_PAGE_NUMBER_MASK;
            unsafe {
                swapper_page_directory[level2_index] =
                    PageTableEntry::new(gigapage_physical_address, flags)
            };
            gigapage_physical_address += GIGA_PAGE_SIZE;
        }
    }

    let swapper_physical_address: PhysicalAddress =
        VirtualAddress::new(&raw const swapper_page_directory as usize).to_kernel_physical();
    Satp::new()
        .set_sv39()
        .with_root_physical_address(swapper_physical_address.as_usize())
        .write();
    unsafe { core::arch::asm!("sfence.vma zero, zero", options(nostack)) };
}
