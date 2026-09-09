use super::constants::{
    KERNEL_PHYSICAL_BASE, KERNEL_VIRTUAL_BASE, PAGE_TABLE_LEVEL2_SHIFT, VIRTUAL_PAGE_NUMBER_MASK,
};
use super::csr::satp::Satp;
use super::page_table::PageTable;
use super::page_table_entry::PageTableEntry;
use super::page_table_entry_flags::PageTableEntryFlags;

#[unsafe(no_mangle)]
pub static mut EARLY_PAGE_DIRECTORY: PageTable = PageTable::empty();

#[unsafe(no_mangle)]
extern "C" fn setup_virtual_memory() {
    unsafe {
        let identity_index: usize =
            (KERNEL_PHYSICAL_BASE >> PAGE_TABLE_LEVEL2_SHIFT) & VIRTUAL_PAGE_NUMBER_MASK;
        let higher_half_index: usize =
            (KERNEL_VIRTUAL_BASE >> PAGE_TABLE_LEVEL2_SHIFT) & VIRTUAL_PAGE_NUMBER_MASK;
        let gigapage_physical_base: usize =
            KERNEL_PHYSICAL_BASE & !((1 << PAGE_TABLE_LEVEL2_SHIFT) - 1);
        let kernel_leaf: PageTableEntryFlags = PageTableEntryFlags::new()
            .valid()
            .read()
            .write()
            .execute()
            .global()
            .accessed()
            .dirty();
        let gigapage_page_table_entry: PageTableEntry =
            PageTableEntry::new(gigapage_physical_base, kernel_leaf);

        let early_page_directory_ptr: *mut PageTable =
            core::ptr::addr_of_mut!(EARLY_PAGE_DIRECTORY);
        PageTable::write_entry(early_page_directory_ptr, identity_index, gigapage_page_table_entry);
        PageTable::write_entry(early_page_directory_ptr, higher_half_index, gigapage_page_table_entry);

        Satp::write(
            Satp::from_bits(0)
                .set_sv39()
                .with_root_physical_address(core::ptr::addr_of!(EARLY_PAGE_DIRECTORY) as usize),
        );
        core::arch::asm!("sfence.vma zero, zero", options(nostack));
    }
}
