use core::ptr::NonNull;

use ferrum_core::linked_list::list::List;

use super::free_block::FreeBlock;

pub struct FreeArea {
    pub list: List<FreeBlock>,
    pub map: *mut usize,
}

unsafe impl Send for FreeArea {}
unsafe impl Sync for FreeArea {}

impl FreeArea {
    pub const fn empty() -> Self {
        Self {
            list: List::new(),
            map: core::ptr::null_mut(),
        }
    }

    pub fn push_front(&mut self, node: NonNull<FreeBlock>) {
        self.list.push_front(node);
    }

    pub fn pop_front(&mut self) -> Option<NonNull<FreeBlock>> {
        self.list.pop_front()
    }

    pub fn remove(&mut self, node: NonNull<FreeBlock>) {
        unsafe { self.list.remove(node) };
    }

    pub fn toggle_and_test_buddy_bit(&mut self, bit_index: usize) -> bool {
        let word_index: usize = bit_index / usize::BITS as usize;
        let bit_offset: usize = bit_index % usize::BITS as usize;
        unsafe {
            *self.map.add(word_index) ^= 1 << bit_offset;
            (*self.map.add(word_index) >> bit_offset) & 1 == 0
        }
    }
}
