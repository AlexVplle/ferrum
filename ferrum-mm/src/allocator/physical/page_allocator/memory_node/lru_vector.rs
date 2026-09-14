use core::ptr::NonNull;
use ferrum_core::linked_list::list::List;
use crate::page::frame::Frame;
use super::lru_list::{LruList, NR_LRU_LISTS};

pub struct LruVector {
    lists: [List<Frame>; NR_LRU_LISTS],
}

impl LruVector {
    pub const fn new() -> Self {
        const EMPTY: List<Frame> = List::new();
        Self {
            lists: [EMPTY; NR_LRU_LISTS],
        }
    }

    pub fn add(&mut self, lru: LruList, ptr: NonNull<Frame>) {
        self.lists[lru as usize].push_back(ptr);
    }

    pub fn delete(&mut self, lru: LruList, ptr: NonNull<Frame>) {
        unsafe { self.lists[lru as usize].remove(ptr) };
    }

    pub fn list(&self, lru: LruList) -> &List<Frame> {
        &self.lists[lru as usize]
    }
}
