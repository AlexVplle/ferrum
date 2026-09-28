pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);
pub const KERNEL_PHYSICAL_BASE: usize = 0x80200000;
pub const PAGE_TABLE_LEVEL0_SHIFT: usize = 12;
pub const PHYSICAL_PAGE_NUMBER_SHIFT: usize = 10;
pub const TLB_FLUSH_ALL_THRESHOLD: usize = 64;
pub const DIRECT_MEMORY_ACCESS_ZONE_END: Option<usize> = None;
pub const MAX_HARTS: usize = 8;
pub const GIGA_PAGE_MASK: usize = !(crate::arch::GIGA_PAGE_SIZE - 1);
pub const PHYSICAL_TO_VIRTUAL_OFFSET: usize =
    crate::arch::KERNEL_VIRTUAL_BASE - crate::arch::KERNEL_PHYSICAL_BASE;
