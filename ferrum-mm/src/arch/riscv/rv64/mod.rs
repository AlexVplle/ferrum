use core::sync::atomic::{AtomicUsize, Ordering};

#[path = "../shared/constants.rs"]
mod common;
pub use common::*;

pub const PAGE_TABLE_ENTRIES: usize = 512;
pub const SV39_PAGE_OFFSET: usize = 0xFFFFFFC000000000;
pub const SV48_PAGE_OFFSET: usize = 0xFFFF800000000000;
pub const SV57_PAGE_OFFSET: usize = 0xFF00000000000000;
pub const SECTION_SIZE_BITS: usize = 27;
pub const MAX_PHYSMEM_BITS: usize = 56;
pub const KERNEL_VIRTUAL_BASE: usize = 0xffffffff80200000;
pub const PAGE_TABLE_LEVEL1_SHIFT: usize = 21;
pub const PAGE_TABLE_LEVEL2_SHIFT: usize = 30;
pub const PAGE_TABLE_LEVEL3_SHIFT: usize = 39;
pub const PAGE_TABLE_LEVEL4_SHIFT: usize = 48;
pub const GIGA_PAGE_SIZE: usize = 1 << PAGE_TABLE_LEVEL2_SHIFT;
pub const GIGA_PAGE_MASK: usize = !(GIGA_PAGE_SIZE - 1);
pub const TERA_PAGE_SIZE: usize = 1 << PAGE_TABLE_LEVEL3_SHIFT;
pub const TERA_PAGE_MASK: usize = !(TERA_PAGE_SIZE - 1);
pub const PETA_PAGE_SIZE: usize = 1 << PAGE_TABLE_LEVEL4_SHIFT;
pub const PETA_PAGE_MASK: usize = !(PETA_PAGE_SIZE - 1);
pub const PHYSICAL_PAGE_NUMBER_MASK: usize = 0x003FFFFFFFFFFC00;
pub const VIRTUAL_PAGE_NUMBER_MASK: usize = 0x1FF;

pub mod paging_mode;
pub use paging_mode::PagingMode;

static PAGING_MODE_STORAGE: AtomicUsize = AtomicUsize::new(PagingMode::Sv39 as usize);
static KERNEL_VIRTUAL_BASE_STORAGE: AtomicUsize = AtomicUsize::new(KERNEL_VIRTUAL_BASE);
static PHYSICAL_TO_VIRTUAL_OFFSET_STORAGE: AtomicUsize =
    AtomicUsize::new(KERNEL_VIRTUAL_BASE - KERNEL_PHYSICAL_BASE);

pub fn paging_mode() -> PagingMode {
    PagingMode::try_from(PAGING_MODE_STORAGE.load(Ordering::Relaxed))
        .unwrap_or(PagingMode::Sv39)
}

pub fn set_paging_mode(mode: PagingMode) {
    PAGING_MODE_STORAGE.store(mode as usize, Ordering::Relaxed);
}

pub fn page_offset() -> usize {
    match paging_mode() {
        PagingMode::Sv39 => SV39_PAGE_OFFSET,
        PagingMode::Sv48 => SV48_PAGE_OFFSET,
        PagingMode::Sv57 => SV57_PAGE_OFFSET,
    }
}

pub fn kernel_virtual_base() -> usize {
    KERNEL_VIRTUAL_BASE_STORAGE.load(Ordering::Relaxed)
}

pub fn set_kernel_virtual_base(value: usize) {
    KERNEL_VIRTUAL_BASE_STORAGE.store(value, Ordering::Relaxed);
}

pub fn physical_to_virtual_offset() -> usize {
    PHYSICAL_TO_VIRTUAL_OFFSET_STORAGE.load(Ordering::Relaxed)
}

pub fn set_physical_to_virtual_offset(value: usize) {
    PHYSICAL_TO_VIRTUAL_OFFSET_STORAGE.store(value, Ordering::Relaxed);
}

pub mod fixmap;
#[path = "../shared/numa.rs"]
pub mod numa;
pub mod paging;
