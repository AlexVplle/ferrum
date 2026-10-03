#[path = "../shared/constants.rs"]
mod common;
pub use common::*;

pub const PAGE_TABLE_ENTRIES: usize = 1024;
pub const PAGE_OFFSET: usize = 0xC0000000;
pub const SECTION_SIZE_BITS: usize = 22;
pub const MAX_PHYSMEM_BITS: usize = 34;
pub const KERNEL_VIRTUAL_BASE: usize = 0xC0200000;
pub const PAGE_TABLE_LEVEL1_SHIFT: usize = 22;
pub const MEGA_PAGE_SIZE: usize = 1 << PAGE_TABLE_LEVEL1_SHIFT;
pub const MEGA_PAGE_MASK: usize = !(MEGA_PAGE_SIZE - 1);
pub const PHYSICAL_PAGE_NUMBER_MASK: usize = 0xFFFFFC00;
pub const VIRTUAL_PAGE_NUMBER_MASK: usize = 0x3FF;

pub mod fixmap;
#[path = "../shared/numa.rs"]
pub mod numa;
pub mod paging;
