use core::ptr::NonNull;
use core::sync::atomic::AtomicUsize;

use super::constants::PAGES_PER_SECTION;
use super::usage::MemSectionUsage;
use crate::arch::PAGE_SHIFT;
use crate::page::frame::Frame;

pub static MAX_PAGE_FRAME_NUMBER: AtomicUsize = AtomicUsize::new(0);
pub static MIN_LOW_PAGE_FRAME_NUMBER: AtomicUsize = AtomicUsize::new(usize::MAX);

pub struct MemorySection {
    pub section_memory_map: Option<NonNull<Frame>>,
    pub base_page_frame_number: usize,
    pub usage: MemSectionUsage,
}

impl MemorySection {
    pub const fn empty() -> Self {
        Self {
            section_memory_map: None,
            base_page_frame_number: 0,
            usage: MemSectionUsage::new(),
        }
    }

    pub fn is_present(&self) -> bool {
        self.section_memory_map.is_some()
    }

    pub fn set_base_page_frame_number(&mut self, base_page_frame_number: usize) {
        self.base_page_frame_number = base_page_frame_number;
    }

    pub fn set_memory_map(&mut self, memory_map: NonNull<Frame>) {
        unsafe {
            for i in 0..PAGES_PER_SECTION {
                memory_map.add(i).write(Frame::empty());
            }
        }
        self.section_memory_map = Some(memory_map);
    }
}

pub fn page_frame_number_to_physical(page_frame_number: usize) -> usize {
    page_frame_number << PAGE_SHIFT
}
