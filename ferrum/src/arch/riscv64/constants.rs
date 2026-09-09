pub const PAGE_TABLE_ENTRIES: usize = 512;
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);
pub const TLB_FLUSH_ALL_THRESHOLD: usize = 64;
pub const SECTION_SIZE_BITS: usize = 27;
pub const MAX_PHYSMEM_BITS: usize = 56;
pub const PAGE_OFFSET: usize = 0xFFFFFFC000000000;

pub const KERNEL_PHYSICAL_BASE: usize = 0x80200000;
pub const KERNEL_VIRTUAL_BASE: usize = 0xffffffff80200000;
pub const PHYSICAL_TO_VIRTUAL_OFFSET: usize = KERNEL_VIRTUAL_BASE.wrapping_sub(KERNEL_PHYSICAL_BASE);

pub const GIGA_PAGE_SIZE: usize = 1 << PAGE_TABLE_LEVEL2_SHIFT;
pub const GIGA_PAGE_MASK: usize = !(GIGA_PAGE_SIZE - 1);
pub const FIXMAP_BASE: usize = (KERNEL_VIRTUAL_BASE & GIGA_PAGE_MASK) - GIGA_PAGE_SIZE;

pub const PHYSICAL_PAGE_NUMBER_SHIFT: usize = 10;
pub const PHYSICAL_PAGE_NUMBER_MASK: usize = 0x003FFFFFFFFFFC00;

pub const VIRTUAL_PAGE_NUMBER_MASK: usize = 0x1FF;
pub const PAGE_TABLE_LEVEL0_SHIFT: usize = 12;
pub const PAGE_TABLE_LEVEL1_SHIFT: usize = 21;
pub const PAGE_TABLE_LEVEL2_SHIFT: usize = 30;
