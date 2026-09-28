use super::constants::KERNEL_LEVEL2_INDEX;
use super::page_table::PageTable;
use super::page_table_entry::PageTableEntry;
use super::page_table_entry_flags::PageTableEntryFlags;
use super::satp::Satp;
use super::tlb::flush_tlb_all;
use crate::arch::{GIGA_PAGE_MASK, PAGE_TABLE_LEVEL2_SHIFT, VIRTUAL_PAGE_NUMBER_MASK};
use crate::physical_address::PhysicalAddress;

unsafe extern "C" {
    static _kernel_start: u8;
}

#[unsafe(no_mangle)]
pub static mut EARLY_PAGE_DIRECTORY: PageTable = PageTable::empty();

pub fn setup_virtual_memory() {
    let early_page_directory_physical_address: usize;
    let kernel_physical_base: usize;
    unsafe {
        core::arch::asm!(
            ".option push",
            ".option nopic",
            "la {0}, EARLY_PAGE_DIRECTORY",
            "la {1}, {kernel_start}",
            ".option pop",
            out(reg) early_page_directory_physical_address,
            out(reg) kernel_physical_base,
            kernel_start = sym _kernel_start,
        );

        let kernel_gigapage_physical_base: PhysicalAddress =
            PhysicalAddress::new(kernel_physical_base & GIGA_PAGE_MASK);
        let flags: PageTableEntryFlags = PageTableEntryFlags::new()
            .valid()
            .read()
            .write()
            .execute()
            .global()
            .accessed()
            .dirty();
        let entry: PageTableEntry = PageTableEntry::new(kernel_gigapage_physical_base, flags);
        let identity_index: usize =
            (kernel_gigapage_physical_base.as_usize() >> PAGE_TABLE_LEVEL2_SHIFT) & VIRTUAL_PAGE_NUMBER_MASK;

        let early_page_directory: *mut PageTableEntry =
            early_page_directory_physical_address as *mut PageTableEntry;
        early_page_directory.add(identity_index).write(entry);
        early_page_directory.add(KERNEL_LEVEL2_INDEX).write(entry);

        Satp::new()
            .set_sv39()
            .with_root_physical_address(early_page_directory_physical_address)
            .write();
        flush_tlb_all();
    }
}
