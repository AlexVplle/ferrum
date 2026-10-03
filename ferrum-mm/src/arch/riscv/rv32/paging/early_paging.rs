use super::constants::KERNEL_LEVEL1_INDEX;
use super::page_table::PageTable;
use super::page_table_entry::PageTableEntry;
use super::page_table_entry_flags::PageTableEntryFlags;
use super::satp::Satp;
use super::tlb::flush_tlb_all;
use crate::arch::{MEGA_PAGE_MASK, MEGA_PAGE_SIZE, PAGE_TABLE_ENTRIES, PAGE_TABLE_LEVEL1_SHIFT, VIRTUAL_PAGE_NUMBER_MASK};
use crate::physical_address::PhysicalAddress;

unsafe extern "C" {
    static _kernel_start: u8;
    static _kernel_end: u8;
}

#[unsafe(no_mangle)]
pub static mut EARLY_PAGE_DIRECTORY: PageTable = PageTable::empty();

pub fn setup_virtual_memory() {
    let early_page_directory_physical_address: usize;
    let kernel_physical_base: usize;
    let kernel_physical_end: usize;
    unsafe {
        core::arch::asm!(
            ".option push",
            ".option nopic",
            "la {0}, EARLY_PAGE_DIRECTORY",
            "la {1}, {kernel_start}",
            "la {2}, {kernel_end}",
            ".option pop",
            out(reg) early_page_directory_physical_address,
            out(reg) kernel_physical_base,
            out(reg) kernel_physical_end,
            kernel_start = sym _kernel_start,
            kernel_end = sym _kernel_end,
        );

        let flags: PageTableEntryFlags = PageTableEntryFlags::new()
            .valid()
            .read()
            .write()
            .execute()
            .global()
            .accessed()
            .dirty();

        let early_page_directory: *mut PageTableEntry =
            early_page_directory_physical_address as *mut PageTableEntry;

        let megapage_base: PhysicalAddress = PhysicalAddress::new(kernel_physical_base & MEGA_PAGE_MASK);
        let megapage_end: PhysicalAddress = PhysicalAddress::new((kernel_physical_end + MEGA_PAGE_SIZE - 1) & MEGA_PAGE_MASK);
        let identity_index: usize = (megapage_base.as_usize() >> PAGE_TABLE_LEVEL1_SHIFT) & VIRTUAL_PAGE_NUMBER_MASK;

        let mut physical_address: PhysicalAddress = megapage_base;
        let mut i: usize = 0;
        while physical_address.as_usize() < megapage_end.as_usize() && KERNEL_LEVEL1_INDEX + i < PAGE_TABLE_ENTRIES {
            let entry: PageTableEntry = PageTableEntry::new(physical_address, flags);
            if i == 0 {
                early_page_directory.add(identity_index).write(entry);
            }
            early_page_directory.add(KERNEL_LEVEL1_INDEX + i).write(entry);
            physical_address += MEGA_PAGE_SIZE;
            i += 1;
        }

        Satp::new()
            .set_sv32()
            .with_root_physical_address(early_page_directory_physical_address)
            .write();
        flush_tlb_all();
    }
}
