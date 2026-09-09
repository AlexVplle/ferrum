use super::constants::GIGAPAGE_PHYSICAL_BASE;
use super::page_table::PageTable;
use super::page_table_entry::PageTableEntry;
use super::page_table_entry_flags::PageTableEntryFlags;
use super::satp::Satp;
use crate::arch::{
    KERNEL_PHYSICAL_BASE, KERNEL_VIRTUAL_BASE, PAGE_TABLE_LEVEL2_SHIFT, VIRTUAL_PAGE_NUMBER_MASK,
};

#[unsafe(no_mangle)]
pub static mut EARLY_PAGE_DIRECTORY: PageTable = PageTable::empty();

pub fn setup_virtual_memory() {
    let physical_address: usize;
    unsafe {
        core::arch::asm!(
            ".option push",
            ".option nopic",
            "la {0}, EARLY_PAGE_DIRECTORY",
            ".option pop",
            out(reg) physical_address,
        );

        let directory: *mut PageTable = physical_address as *mut PageTable;
        let flags: PageTableEntryFlags = PageTableEntryFlags::new()
            .valid()
            .read()
            .write()
            .execute()
            .global()
            .accessed()
            .dirty();
        let entry: PageTableEntry = PageTableEntry::new(GIGAPAGE_PHYSICAL_BASE, flags);
        let identity_index: usize =
            (KERNEL_PHYSICAL_BASE >> PAGE_TABLE_LEVEL2_SHIFT) & VIRTUAL_PAGE_NUMBER_MASK;
        let higher_half_index: usize =
            (KERNEL_VIRTUAL_BASE >> PAGE_TABLE_LEVEL2_SHIFT) & VIRTUAL_PAGE_NUMBER_MASK;
        directory.cast::<PageTableEntry>().add(identity_index).write(entry);
        directory.cast::<PageTableEntry>().add(higher_half_index).write(entry);

        Satp::new()
            .set_sv39()
            .with_root_physical_address(physical_address)
            .write();
        core::arch::asm!("sfence.vma zero, zero", options(nostack));
    }
}
