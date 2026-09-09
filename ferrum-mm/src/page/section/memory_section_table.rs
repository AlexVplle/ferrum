use core::sync::atomic::{AtomicPtr, Ordering};

use super::constants::{
    NR_SECTION_ROOTS, PAGE_FRAME_NUMBER_SECTION_SHIFT, SECTIONS_PER_ROOT, SECTIONS_PER_ROOT_BITS,
};
use super::memory_section::MemorySection;
use crate::memory_block::MEMORY_BLOCK;
use crate::page::frame::Frame;

pub(crate) struct MemorySectionTable([AtomicPtr<MemorySection>; NR_SECTION_ROOTS]);

impl MemorySectionTable {
    pub(crate) const fn new() -> Self {
        Self([const { AtomicPtr::new(core::ptr::null_mut()) }; NR_SECTION_ROOTS])
    }

    pub(crate) fn section_number_for_page_frame_number(&self, page_frame_number: usize) -> usize {
        page_frame_number >> PAGE_FRAME_NUMBER_SECTION_SHIFT
    }

    pub(crate) fn base_page_frame_number_for_section(&self, section_number: usize) -> usize {
        section_number << PAGE_FRAME_NUMBER_SECTION_SHIFT
    }

    pub(crate) fn is_root_allocated(&self, root: usize) -> bool {
        !self.0[root].load(Ordering::Acquire).is_null()
    }

    pub(crate) fn alloc_root(&self, root: usize) {
        let root_storage: core::ptr::NonNull<MemorySection> = unsafe {
            (*(&raw mut MEMORY_BLOCK))
                .alloc(
                    SECTIONS_PER_ROOT * core::mem::size_of::<MemorySection>(),
                    core::mem::align_of::<MemorySection>(),
                )
                .expect("alloc_root: failed to allocate root storage")
                .to_virtual()
                .as_non_null::<MemorySection>()
        };
        self.0[root].store(root_storage.as_ptr(), Ordering::Release);
    }

    pub(crate) unsafe fn section_mut(&self, section_number: usize) -> &mut MemorySection {
        let root: usize = section_number >> SECTIONS_PER_ROOT_BITS;
        let offset: usize = section_number & (SECTIONS_PER_ROOT - 1);
        unsafe { &mut *self.0[root].load(Ordering::Acquire).add(offset) }
    }

    pub fn page_frame_number_to_page(&self, page_frame_number: usize) -> *mut Frame {
        let section_number: usize = self.section_number_for_page_frame_number(page_frame_number);
        let section: &MemorySection = &self[section_number];
        let page_frame_number_in_section: usize =
            page_frame_number - section.base_page_frame_number;
        unsafe {
            section
                .section_memory_map
                .unwrap()
                .add(page_frame_number_in_section)
                .as_ptr()
        }
    }

    pub fn page_to_page_frame_number(&self, page: *const Frame, section_number: usize) -> usize {
        let section: &MemorySection = &self[section_number];
        section.base_page_frame_number
            + (page as usize - section.section_memory_map.unwrap().as_ptr() as usize)
                / core::mem::size_of::<Frame>()
    }
}

impl core::ops::Index<usize> for MemorySectionTable {
    type Output = MemorySection;
    fn index(&self, section_number: usize) -> &MemorySection {
        let root: usize = section_number >> SECTIONS_PER_ROOT_BITS;
        let offset: usize = section_number & (SECTIONS_PER_ROOT - 1);
        unsafe { &*self.0[root].load(Ordering::Acquire).add(offset) }
    }
}

pub static MEM_SECTION: MemorySectionTable = MemorySectionTable::new();
