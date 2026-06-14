use super::constants::{
    KERNEL_VIRTUAL_BASE, PAGE_OFFSET, PAGE_TABLE_LEVEL2_SHIFT, PHYSICAL_TO_VIRTUAL_OFFSET,
    VIRTUAL_PAGE_NUMBER_MASK,
};
use super::csr::satp::Satp;
use super::early_paging::EARLY_PAGE_DIRECTORY;
use super::fixmap::FIXMAP_L2_INDEX;
use super::page_table::PageTable;
use super::page_table_entry::PageTableEntry;
use super::page_table_entry_flags::PageTableEntryFlags;
use crate::memory_management::early::memory_map_entry::MemoryMapEntry;

#[unsafe(no_mangle)]
static mut swapper_page_directory: PageTable = PageTable::empty();

pub unsafe fn setup_direct_map(regions: &[MemoryMapEntry]) {
    unsafe {
        let page_directory: *mut PageTable = core::ptr::addr_of_mut!(swapper_page_directory);

        let early_page_directory: *const PageTable = core::ptr::addr_of!(EARLY_PAGE_DIRECTORY);
        let kernel_index: usize =
            (KERNEL_VIRTUAL_BASE >> PAGE_TABLE_LEVEL2_SHIFT) & VIRTUAL_PAGE_NUMBER_MASK;
        let kernel_entry: PageTableEntry = *(*early_page_directory).entry(kernel_index);
        PageTable::write_entry(page_directory, kernel_index, kernel_entry);

        let fixmap_entry: PageTableEntry = *(*early_page_directory).entry(FIXMAP_L2_INDEX);
        PageTable::write_entry(page_directory, FIXMAP_L2_INDEX, fixmap_entry);

        let flags: PageTableEntryFlags = PageTableEntryFlags::new()
            .valid()
            .read()
            .write()
            .execute()
            .global()
            .accessed()
            .dirty();

        let giga_size: usize = 1 << PAGE_TABLE_LEVEL2_SHIFT;

        for region in regions {
            let start: usize = region.base.as_usize() & !(giga_size - 1);
            let end: usize =
                (region.base.as_usize() + region.size + giga_size - 1) & !(giga_size - 1);

            let mut giga_pa: usize = start;
            while giga_pa < end {
                let direct_map_va: usize = PAGE_OFFSET + giga_pa;
                let l2_index: usize =
                    (direct_map_va >> PAGE_TABLE_LEVEL2_SHIFT) & VIRTUAL_PAGE_NUMBER_MASK;
                PageTable::write_entry(
                    page_directory,
                    l2_index,
                    PageTableEntry::new(giga_pa, flags),
                );
                giga_pa += giga_size;
            }
        }

        let swapper_physical_address: usize = (core::ptr::addr_of!(swapper_page_directory) as usize)
            .wrapping_sub(PHYSICAL_TO_VIRTUAL_OFFSET);
        Satp::write(
            Satp::from_bits(0)
                .set_sv39()
                .with_root_physical_address(swapper_physical_address),
        );
        core::arch::asm!("sfence.vma zero, zero", options(nostack));
    }
}
