pub use crate::arch::{MAX_PHYSMEM_BITS, SECTION_SIZE_BITS};
pub use crate::arch::{PAGE_MASK, PAGE_SHIFT, PAGE_SIZE};

pub const SECTION_SIZE: usize = 1 << SECTION_SIZE_BITS;
pub const PAGES_PER_SECTION: usize = SECTION_SIZE / PAGE_SIZE;
pub const MAX_SECTIONS: usize = 1 << (MAX_PHYSMEM_BITS - SECTION_SIZE_BITS);

pub const PAGE_FRAME_NUMBER_SECTION_SHIFT: usize = SECTION_SIZE_BITS - PAGE_SHIFT;
pub const SECTIONS_PER_ROOT_BITS: usize = 16;
pub const SECTIONS_PER_ROOT: usize = 1 << SECTIONS_PER_ROOT_BITS;
pub const NR_SECTION_ROOTS: usize = MAX_SECTIONS / SECTIONS_PER_ROOT;

pub const ZONE_SHIFT: usize = 6;
pub const ZONE_MASK: usize = 0x3 << ZONE_SHIFT;

pub const NODE_SHIFT: usize = 8;
pub const NODE_MASK: usize = 0xF << NODE_SHIFT;
