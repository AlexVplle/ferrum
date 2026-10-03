use super::super::fixmap::FIXMAP_LEVEL1_INDEX;
use super::constants::KERNEL_LEVEL1_INDEX;
use super::early_paging::EARLY_PAGE_DIRECTORY;
use super::page_table::PageTable;
use super::page_table_entry::PageTableEntry;
use super::page_table_entry_flags::PageTableEntryFlags;
use super::satp::Satp;
use super::tlb::flush_tlb_all;
use crate::arch::{MEGA_PAGE_MASK, MEGA_PAGE_SIZE, PAGE_TABLE_LEVEL1_SHIFT, VIRTUAL_PAGE_NUMBER_MASK};
use crate::memory_block::{MemoryBlockRegion, MEMORY_BLOCK};
use crate::physical_address::PhysicalAddress;
use crate::virtual_address::VirtualAddress;

#[unsafe(no_mangle)]
static mut swapper_page_directory: PageTable = PageTable::empty();

pub fn setup_direct_map() {
    let regions: &[MemoryBlockRegion] = unsafe { (*(&raw const MEMORY_BLOCK)).memory_regions() };
    let mut i: usize = 0;
    while KERNEL_LEVEL1_INDEX + i < crate::arch::PAGE_TABLE_ENTRIES {
        let entry: PageTableEntry = unsafe { EARLY_PAGE_DIRECTORY[KERNEL_LEVEL1_INDEX + i] };
        if !entry.is_valid() {
            break;
        }
        unsafe { swapper_page_directory[KERNEL_LEVEL1_INDEX + i] = entry };
        i += 1;
    }
    unsafe {
        swapper_page_directory[FIXMAP_LEVEL1_INDEX] = EARLY_PAGE_DIRECTORY[FIXMAP_LEVEL1_INDEX]
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
        let start: PhysicalAddress = PhysicalAddress::new(region.base.as_usize() & MEGA_PAGE_MASK);
        let end: PhysicalAddress =
            PhysicalAddress::new((region.base.as_usize() + region.size + MEGA_PAGE_SIZE - 1) & MEGA_PAGE_MASK);

        let mut megapage_physical_address: PhysicalAddress = start;
        while megapage_physical_address.as_usize() < end.as_usize() {
            let direct_map_virtual_address: VirtualAddress = megapage_physical_address.to_virtual();
            let level1_index: usize = (direct_map_virtual_address.as_usize()
                >> PAGE_TABLE_LEVEL1_SHIFT)
                & VIRTUAL_PAGE_NUMBER_MASK;
            unsafe {
                swapper_page_directory[level1_index] =
                    PageTableEntry::new(megapage_physical_address, flags)
            };
            megapage_physical_address += MEGA_PAGE_SIZE;
        }
    }

    let swapper_physical_address: PhysicalAddress =
        VirtualAddress::new(&raw const swapper_page_directory as usize).to_kernel_physical();
    Satp::new()
        .set_sv32()
        .with_root_physical_address(swapper_physical_address.as_usize())
        .write();
    flush_tlb_all();
}
