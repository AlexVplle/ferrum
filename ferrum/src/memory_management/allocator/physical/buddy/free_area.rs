use core::pin::Pin;
use crate::data_structures::list_head::ListHead;

pub struct FreeArea {
    pub head: ListHead,
    pub map: *mut usize,
}

unsafe impl Send for FreeArea {}
unsafe impl Sync for FreeArea {}

impl FreeArea {
    pub const fn empty() -> Self {
        Self {
            head: ListHead::null(),
            map: core::ptr::null_mut(),
        }
    }

    fn ensure_init(&mut self) {
        if !self.head.is_initialized() {
            unsafe { Pin::new_unchecked(&mut self.head) }.init();
        }
    }

    pub unsafe fn push_front(&mut self, node: *mut ListHead) {
        self.ensure_init();
        let head_pin: Pin<&mut ListHead> = unsafe { Pin::new_unchecked(&mut self.head) };
        let node_pin: Pin<&mut ListHead> = unsafe { Pin::new_unchecked(&mut *node) };
        head_pin.insert_after(node_pin);
    }

    pub fn pop_front(&mut self) -> Option<*mut ListHead> {
        self.ensure_init();
        if self.head.is_empty() {
            return None;
        }
        let node: *mut ListHead = self.head.next_raw();
        unsafe { Pin::new_unchecked(&mut *node) }.remove();
        Some(node)
    }

    pub unsafe fn toggle_and_test_buddy_bit(&mut self, bit_index: usize) -> bool {
        let word_index: usize = bit_index / usize::BITS as usize;
        let bit_offset: usize = bit_index % usize::BITS as usize;
        unsafe {
            *self.map.add(word_index) ^= 1 << bit_offset;
            (*self.map.add(word_index) >> bit_offset) & 1 == 0
        }
    }
}
