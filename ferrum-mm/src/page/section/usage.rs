use super::constants::{MIGRATE_SKIP, MIGRATETYPE_MASK, NR_PAGEBLOCK_BITS, PAGE_SECTION_MASK, PAGEBLOCK_FLAGS_WORDS, SUBSECTIONS_PER_SECTION};
use crate::allocator::physical::buddy::constants::PAGEBLOCK_ORDER;
use crate::migrate_type::MigrateType;

pub struct MemorySectionUsage {
    pub subsection_map: usize,
    pub pageblock_flags: [usize; PAGEBLOCK_FLAGS_WORDS],
}

impl MemorySectionUsage {
    pub const fn new() -> Self {
        Self {
            subsection_map: 0,
            pageblock_flags: [0; PAGEBLOCK_FLAGS_WORDS],
        }
    }

    pub fn is_subsection_present(&self, index: usize) -> bool {
        debug_assert!(index < SUBSECTIONS_PER_SECTION);
        self.subsection_map & (1 << index) != 0
    }

    pub fn set_subsection_present(&mut self, index: usize) {
        debug_assert!(index < SUBSECTIONS_PER_SECTION);
        self.subsection_map |= 1 << index;
    }

    fn page_frame_number_to_bit_index(page_frame_number: usize) -> usize {
        ((page_frame_number & !PAGE_SECTION_MASK) >> PAGEBLOCK_ORDER) * NR_PAGEBLOCK_BITS
    }

    pub fn get_page_frame_number_block_flags_mask(&self, page_frame_number: usize, mask: usize) -> usize {
        let bit_index: usize = Self::page_frame_number_to_bit_index(page_frame_number);
        let word: usize = self.pageblock_flags[bit_index / usize::BITS as usize];
        (word >> (bit_index % usize::BITS as usize)) & mask
    }

    pub fn get_pageblock_migratetype(&self, page_frame_number: usize) -> MigrateType {
        match self.get_page_frame_number_block_flags_mask(page_frame_number, MIGRATETYPE_MASK) {
            0 => MigrateType::Unmovable,
            1 => MigrateType::Movable,
            2 => MigrateType::Reclaimable,
            3 => MigrateType::HighAtomic,
            4 => MigrateType::ContiguousMemoryAllocator,
            _ => MigrateType::Isolate,
        }
    }

    pub fn set_page_frame_number_block_flags_mask(&mut self, page_frame_number: usize, flags: usize, mask: usize) {
        let bit_index: usize = Self::page_frame_number_to_bit_index(page_frame_number);
        let word_index: usize = bit_index / usize::BITS as usize;
        let bit_offset: usize = bit_index % usize::BITS as usize;
        let shifted_mask: usize = mask << bit_offset;
        let shifted_flags: usize = flags << bit_offset;
        self.pageblock_flags[word_index] =
            (self.pageblock_flags[word_index] & !shifted_mask) | shifted_flags;
    }

    pub fn get_pageblock_skip(&self, page_frame_number: usize) -> bool {
        self.get_page_frame_number_block_flags_mask(page_frame_number, MIGRATE_SKIP) != 0
    }

    pub fn set_pageblock_skip(&mut self, page_frame_number: usize, skip: bool) {
        self.set_page_frame_number_block_flags_mask(
            page_frame_number,
            (skip as usize) * MIGRATE_SKIP,
            MIGRATE_SKIP,
        );
    }

    pub fn set_pageblock_migratetype(&mut self, page_frame_number: usize, migrate_type: MigrateType) {
        self.set_page_frame_number_block_flags_mask(
            page_frame_number,
            migrate_type as usize,
            MIGRATETYPE_MASK,
        );
    }
}
