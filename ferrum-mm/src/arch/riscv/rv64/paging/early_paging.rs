use super::constants::{KERNEL_LEVEL2_INDEX, KERNEL_LEVEL3_INDEX, KERNEL_LEVEL4_INDEX};
use super::page_table::PageTable;
use super::page_table_entry::PageTableEntry;
use super::page_table_entry_flags::PageTableEntryFlags;
use super::satp::Satp;
use super::tlb::flush_tlb_all;
use crate::arch::{
    GIGA_PAGE_MASK, PAGE_TABLE_LEVEL2_SHIFT, PETA_PAGE_MASK, TERA_PAGE_MASK,
    VIRTUAL_PAGE_NUMBER_MASK,
};
use crate::arch::{set_paging_mode, PagingMode};
use crate::physical_address::PhysicalAddress;

unsafe extern "C" {
    static _kernel_start: u8;
}

#[unsafe(no_mangle)]
pub static mut EARLY_PAGE_DIRECTORY: PageTable = PageTable::empty();
#[unsafe(no_mangle)]
pub static mut EARLY_LEVEL2_TABLE: PageTable = PageTable::empty();
#[unsafe(no_mangle)]
pub static mut EARLY_LEVEL3_TABLE: PageTable = PageTable::empty();

pub unsafe fn early_level2_table() -> &'static mut PageTable {
    match crate::arch::paging_mode() {
        PagingMode::Sv39 => &mut *(&raw mut EARLY_PAGE_DIRECTORY),
        _ => &mut *(&raw mut EARLY_LEVEL2_TABLE),
    }
}

fn probe_satp_mode(satp: Satp) -> bool {
    satp.write();
    let accepted: bool = Satp::read().mode() == satp.mode();
    Satp::bare().write();
    flush_tlb_all();
    accepted
}

pub fn setup_virtual_memory() {
    let early_page_directory_physical: usize;
    let early_level2_table_physical: usize;
    let early_level3_table_physical: usize;
    let kernel_physical_base: usize;

    unsafe {
        core::arch::asm!(
            ".option push",
            ".option nopic",
            "la {0}, {early_page_directory}",
            "la {1}, {early_level2_table}",
            "la {2}, {early_level3_table}",
            "la {3}, {kernel_start}",
            ".option pop",
            out(reg) early_page_directory_physical,
            out(reg) early_level2_table_physical,
            out(reg) early_level3_table_physical,
            out(reg) kernel_physical_base,
            early_page_directory = sym EARLY_PAGE_DIRECTORY,
            early_level2_table = sym EARLY_LEVEL2_TABLE,
            early_level3_table = sym EARLY_LEVEL3_TABLE,
            kernel_start = sym _kernel_start,
        );
    }

    let leaf: PageTableEntryFlags = PageTableEntryFlags::new()
        .valid()
        .read()
        .write()
        .execute()
        .global()
        .accessed()
        .dirty();
    let non_leaf: PageTableEntryFlags = PageTableEntryFlags::new().valid();

    let early_page_directory: *mut PageTableEntry =
        early_page_directory_physical as *mut PageTableEntry;
    let early_level2_table: *mut PageTableEntry =
        early_level2_table_physical as *mut PageTableEntry;
    let early_level3_table: *mut PageTableEntry =
        early_level3_table_physical as *mut PageTableEntry;

    unsafe {
        early_page_directory
            .add(0)
            .write(PageTableEntry::new(PhysicalAddress::new(0), leaf));
    }

    let mode: PagingMode = if probe_satp_mode(
        Satp::new()
            .set_sv57()
            .with_root_physical_address(early_page_directory_physical),
    ) {
        PagingMode::Sv57
    } else if probe_satp_mode(
        Satp::new()
            .set_sv48()
            .with_root_physical_address(early_page_directory_physical),
    ) {
        PagingMode::Sv48
    } else {
        PagingMode::Sv39
    };
    set_paging_mode(mode);

    let kernel_gigapage_base: usize = kernel_physical_base & GIGA_PAGE_MASK;
    let gigapage_entry: PageTableEntry =
        PageTableEntry::new(PhysicalAddress::new(kernel_gigapage_base), leaf);

    unsafe {
        match mode {
            PagingMode::Sv57 => {
                early_level2_table.add(KERNEL_LEVEL2_INDEX).write(gigapage_entry);
                early_level3_table.add(KERNEL_LEVEL3_INDEX).write(PageTableEntry::new(
                    PhysicalAddress::new(early_level2_table_physical),
                    non_leaf,
                ));
                early_page_directory.add(KERNEL_LEVEL4_INDEX).write(PageTableEntry::new(
                    PhysicalAddress::new(early_level3_table_physical),
                    non_leaf,
                ));
                early_page_directory.add(0).write(PageTableEntry::new(
                    PhysicalAddress::new(kernel_physical_base & PETA_PAGE_MASK),
                    leaf,
                ));
                Satp::new()
                    .set_sv57()
                    .with_root_physical_address(early_page_directory_physical)
                    .write();
            }
            PagingMode::Sv48 => {
                early_level2_table.add(KERNEL_LEVEL2_INDEX).write(gigapage_entry);
                early_page_directory.add(KERNEL_LEVEL3_INDEX).write(PageTableEntry::new(
                    PhysicalAddress::new(early_level2_table_physical),
                    non_leaf,
                ));
                early_page_directory.add(0).write(PageTableEntry::new(
                    PhysicalAddress::new(kernel_physical_base & TERA_PAGE_MASK),
                    leaf,
                ));
                Satp::new()
                    .set_sv48()
                    .with_root_physical_address(early_page_directory_physical)
                    .write();
            }
            PagingMode::Sv39 => {
                let identity_index: usize =
                    (kernel_gigapage_base >> PAGE_TABLE_LEVEL2_SHIFT) & VIRTUAL_PAGE_NUMBER_MASK;
                early_page_directory.add(KERNEL_LEVEL2_INDEX).write(gigapage_entry);
                early_page_directory.add(identity_index).write(gigapage_entry);
                Satp::new()
                    .set_sv39()
                    .with_root_physical_address(early_page_directory_physical)
                    .write();
            }
        }
        flush_tlb_all();
    }
}
