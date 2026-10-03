use super::super::fixmap::FIXMAP_LEVEL2_INDEX;
use super::constants::{KERNEL_LEVEL2_INDEX, KERNEL_LEVEL3_INDEX, KERNEL_LEVEL4_INDEX};
use super::early_paging::EARLY_PAGE_DIRECTORY;
use super::page_table::PageTable;
use super::page_table_entry::PageTableEntry;
use super::page_table_entry_flags::PageTableEntryFlags;
use super::satp::Satp;
use super::tlb::flush_tlb_all;
use crate::arch::{
    GIGA_PAGE_MASK, GIGA_PAGE_SIZE, PAGE_TABLE_LEVEL2_SHIFT, PAGE_TABLE_LEVEL3_SHIFT,
    PAGE_TABLE_LEVEL4_SHIFT, TERA_PAGE_MASK, TERA_PAGE_SIZE, VIRTUAL_PAGE_NUMBER_MASK,
};
use crate::arch::{paging_mode, PagingMode};
use crate::memory_block::{MemoryBlockRegion, MEMORY_BLOCK};
use crate::physical_address::PhysicalAddress;
use crate::virtual_address::VirtualAddress;

#[unsafe(no_mangle)]
static mut SWAPPER_PAGE_DIRECTORY: PageTable = PageTable::empty();
static mut SWAPPER_DIRECT_LEVEL3_TABLE: PageTable = PageTable::empty();

pub fn setup_direct_map() {
    let regions: &[MemoryBlockRegion] = unsafe { (*(&raw const MEMORY_BLOCK)).memory_regions() };

    let leaf: PageTableEntryFlags = PageTableEntryFlags::new()
        .valid()
        .read()
        .write()
        .execute()
        .global()
        .accessed()
        .dirty();
    let non_leaf: PageTableEntryFlags = PageTableEntryFlags::new().valid();

    match paging_mode() {
        PagingMode::Sv57 => {
            unsafe {
                SWAPPER_PAGE_DIRECTORY[KERNEL_LEVEL4_INDEX] =
                    EARLY_PAGE_DIRECTORY[KERNEL_LEVEL4_INDEX];
            }
            let p3_physical: PhysicalAddress =
                VirtualAddress::new(&raw const SWAPPER_DIRECT_LEVEL3_TABLE as usize)
                    .to_kernel_physical();
            for region in regions {
                let start: PhysicalAddress =
                    PhysicalAddress::new(region.base.as_usize() & TERA_PAGE_MASK);
                let end: PhysicalAddress = PhysicalAddress::new(
                    (region.base.as_usize() + region.size + TERA_PAGE_SIZE - 1) & TERA_PAGE_MASK,
                );
                let mut terapage_physical: PhysicalAddress = start;
                while terapage_physical.as_usize() < end.as_usize() {
                    let direct_map_virtual: VirtualAddress = terapage_physical.to_virtual();
                    let level4_index: usize = (direct_map_virtual.as_usize()
                        >> PAGE_TABLE_LEVEL4_SHIFT)
                        & VIRTUAL_PAGE_NUMBER_MASK;
                    let level3_index: usize = (direct_map_virtual.as_usize()
                        >> PAGE_TABLE_LEVEL3_SHIFT)
                        & VIRTUAL_PAGE_NUMBER_MASK;
                    unsafe {
                        SWAPPER_PAGE_DIRECTORY[level4_index] =
                            PageTableEntry::new(p3_physical, non_leaf);
                        SWAPPER_DIRECT_LEVEL3_TABLE[level3_index] =
                            PageTableEntry::new(terapage_physical, leaf);
                    }
                    terapage_physical += TERA_PAGE_SIZE;
                }
            }
            let swapper_physical: PhysicalAddress =
                VirtualAddress::new(&raw const SWAPPER_PAGE_DIRECTORY as usize)
                    .to_kernel_physical();
            Satp::new()
                .set_sv57()
                .with_root_physical_address(swapper_physical.as_usize())
                .write();
        }
        PagingMode::Sv48 => {
            unsafe {
                SWAPPER_PAGE_DIRECTORY[KERNEL_LEVEL3_INDEX] =
                    EARLY_PAGE_DIRECTORY[KERNEL_LEVEL3_INDEX];
            }
            for region in regions {
                let start: PhysicalAddress =
                    PhysicalAddress::new(region.base.as_usize() & TERA_PAGE_MASK);
                let end: PhysicalAddress = PhysicalAddress::new(
                    (region.base.as_usize() + region.size + TERA_PAGE_SIZE - 1) & TERA_PAGE_MASK,
                );
                let mut terapage_physical: PhysicalAddress = start;
                while terapage_physical.as_usize() < end.as_usize() {
                    let direct_map_virtual: VirtualAddress = terapage_physical.to_virtual();
                    let level3_index: usize = (direct_map_virtual.as_usize()
                        >> PAGE_TABLE_LEVEL3_SHIFT)
                        & VIRTUAL_PAGE_NUMBER_MASK;
                    unsafe {
                        SWAPPER_PAGE_DIRECTORY[level3_index] =
                            PageTableEntry::new(terapage_physical, leaf);
                    }
                    terapage_physical += TERA_PAGE_SIZE;
                }
            }
            let swapper_physical: PhysicalAddress =
                VirtualAddress::new(&raw const SWAPPER_PAGE_DIRECTORY as usize)
                    .to_kernel_physical();
            Satp::new()
                .set_sv48()
                .with_root_physical_address(swapper_physical.as_usize())
                .write();
        }
        PagingMode::Sv39 => {
            unsafe {
                SWAPPER_PAGE_DIRECTORY[KERNEL_LEVEL2_INDEX] =
                    EARLY_PAGE_DIRECTORY[KERNEL_LEVEL2_INDEX];
                SWAPPER_PAGE_DIRECTORY[FIXMAP_LEVEL2_INDEX] =
                    EARLY_PAGE_DIRECTORY[FIXMAP_LEVEL2_INDEX];
            }
            for region in regions {
                let start: PhysicalAddress =
                    PhysicalAddress::new(region.base.as_usize() & GIGA_PAGE_MASK);
                let end: PhysicalAddress = PhysicalAddress::new(
                    (region.base.as_usize() + region.size + GIGA_PAGE_SIZE - 1) & GIGA_PAGE_MASK,
                );
                let mut gigapage_physical: PhysicalAddress = start;
                while gigapage_physical.as_usize() < end.as_usize() {
                    let direct_map_virtual: VirtualAddress = gigapage_physical.to_virtual();
                    let level2_index: usize = (direct_map_virtual.as_usize()
                        >> PAGE_TABLE_LEVEL2_SHIFT)
                        & VIRTUAL_PAGE_NUMBER_MASK;
                    unsafe {
                        SWAPPER_PAGE_DIRECTORY[level2_index] =
                            PageTableEntry::new(gigapage_physical, leaf);
                    }
                    gigapage_physical += GIGA_PAGE_SIZE;
                }
            }
            let swapper_physical: PhysicalAddress =
                VirtualAddress::new(&raw const SWAPPER_PAGE_DIRECTORY as usize)
                    .to_kernel_physical();
            Satp::new()
                .set_sv39()
                .with_root_physical_address(swapper_physical.as_usize())
                .write();
        }
    }
    flush_tlb_all();
}
