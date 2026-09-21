use core::ptr::NonNull;

use ferrum_core::linked_list::list::List;

use crate::migrate_type::{MigrateType, NR_MIGRATE_TYPES};

use super::free_block::FreeBlock;

pub struct FreeArea {
    pub lists: [List<FreeBlock>; NR_MIGRATE_TYPES],
    pub map: *mut usize,
}

unsafe impl Send for FreeArea {}
unsafe impl Sync for FreeArea {}

impl FreeArea {
    pub const fn empty() -> Self {
        const EMPTY_LIST: List<FreeBlock> = List::new();
        Self {
            lists: [EMPTY_LIST; NR_MIGRATE_TYPES],
            map: core::ptr::null_mut(),
        }
    }

    pub fn push_front(&mut self, node: NonNull<FreeBlock>, migrate_type: MigrateType) {
        self.lists[migrate_type as usize].push_front(node);
    }

    pub fn pop_front(&mut self, migrate_type: MigrateType) -> Option<NonNull<FreeBlock>> {
        self.lists[migrate_type as usize].pop_front()
    }

    pub fn remove(&mut self, node: NonNull<FreeBlock>, migrate_type: MigrateType) {
        unsafe { self.lists[migrate_type as usize].remove(node) };
    }

    pub fn is_empty(&self, migrate_type: MigrateType) -> bool {
        self.lists[migrate_type as usize].is_empty()
    }

    pub fn toggle_buddy_bit(&mut self, bit_index: usize) {
        let word_index: usize = bit_index / usize::BITS as usize;
        let bit_offset: usize = bit_index % usize::BITS as usize;
        unsafe { *self.map.add(word_index) ^= 1 << bit_offset };
    }

    pub fn test_buddy_bit(&self, bit_index: usize) -> bool {
        let word_index: usize = bit_index / usize::BITS as usize;
        let bit_offset: usize = bit_index % usize::BITS as usize;
        unsafe { ((*self.map.add(word_index)) >> bit_offset) & 1 == 1 }
    }
}
