pub use crate::arch::{MAX_PHYSMEM_BITS, SECTION_SIZE_BITS};

pub const SECTION_SIZE: usize = 1 << SECTION_SIZE_BITS;
pub const PAGES_PER_SECTION: usize = SECTION_SIZE / crate::arch::PAGE_SIZE;
pub const MAX_SECTIONS: usize = 1 << (MAX_PHYSMEM_BITS - SECTION_SIZE_BITS);

pub const PAGE_FRAME_NUMBER_SECTION_SHIFT: usize = SECTION_SIZE_BITS - crate::arch::PAGE_SHIFT;
pub const PAGE_SECTION_MASK: usize = !((1 << PAGE_FRAME_NUMBER_SECTION_SHIFT) - 1);
pub const SECTIONS_PER_ROOT_BITS: usize = 16;
pub const SECTIONS_PER_ROOT: usize = if MAX_SECTIONS < (1 << SECTIONS_PER_ROOT_BITS) {
    MAX_SECTIONS
} else {
    1 << SECTIONS_PER_ROOT_BITS
};
pub const SECTION_ROOT_MASK: usize = SECTIONS_PER_ROOT - 1;
pub const NR_SECTION_ROOTS: usize = MAX_SECTIONS / SECTIONS_PER_ROOT;

pub const MIGRATE_TYPE_BITS: usize = 3;
pub const MIGRATE_SKIP_BIT: usize = MIGRATE_TYPE_BITS;
pub const MIGRATE_SKIP: usize = 1 << MIGRATE_SKIP_BIT;
pub const NR_PAGEBLOCK_BITS: usize = MIGRATE_TYPE_BITS + 1;
pub const MIGRATETYPE_MASK: usize = (1 << MIGRATE_TYPE_BITS) - 1;
pub const SECTION_BLOCKFLAGS_BITS: usize =
    (1 << (PAGE_FRAME_NUMBER_SECTION_SHIFT - crate::allocator::physical::buddy::constants::PAGEBLOCK_ORDER))
        * NR_PAGEBLOCK_BITS;
pub const PAGEBLOCK_FLAGS_WORDS: usize =
    (SECTION_BLOCKFLAGS_BITS + usize::BITS as usize - 1) / usize::BITS as usize;
pub const USEMAP_SIZE: usize = PAGEBLOCK_FLAGS_WORDS * core::mem::size_of::<usize>();

pub const SUBSECTION_SHIFT: usize = 21;
pub const SUBSECTION_SIZE: usize = 1 << SUBSECTION_SHIFT;
pub const SUBSECTIONS_PER_SECTION: usize = SECTION_SIZE / SUBSECTION_SIZE;
pub const PAGES_PER_SUBSECTION: usize = PAGES_PER_SECTION / SUBSECTIONS_PER_SECTION;
