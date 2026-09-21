use core::sync::atomic::{AtomicPtr, Ordering};

use super::constants::{
    MAX_SECTIONS, NR_SECTION_ROOTS, PAGE_FRAME_NUMBER_SECTION_SHIFT, SECTION_ROOT_MASK,
    SECTIONS_PER_ROOT,
};
use super::memory_section::MemorySection;
use crate::memory_block::MEMORY_BLOCK;
use crate::migrate_type::MigrateType;
use crate::page::frame::Frame;

pub struct MemorySectionTable([AtomicPtr<MemorySection>; NR_SECTION_ROOTS]);

impl MemorySectionTable {
    pub(crate) const fn new() -> Self {
        Self([const { AtomicPtr::new(core::ptr::null_mut()) }; NR_SECTION_ROOTS])
    }

    pub fn page_frame_number_to_section_number(&self, page_frame_number: usize) -> usize {
        page_frame_number >> PAGE_FRAME_NUMBER_SECTION_SHIFT
    }

    pub fn page_frame_number_to_section(&self, page_frame_number: usize) -> &MemorySection {
        &self[self.page_frame_number_to_section_number(page_frame_number)]
    }

    pub fn base_page_frame_number_for_section(&self, section_number: usize) -> usize {
        section_number << PAGE_FRAME_NUMBER_SECTION_SHIFT
    }

    pub fn section_number_to_section(&self, section_number: usize) -> Option<&MemorySection> {
        let root: usize = Self::section_root(section_number);
        if root >= NR_SECTION_ROOTS || !self.is_root_allocated(root) {
            return None;
        }
        Some(&self[section_number])
    }

    pub fn section_root(section_number: usize) -> usize {
        section_number / SECTIONS_PER_ROOT
    }

    pub fn is_root_allocated(&self, root: usize) -> bool {
        !self.0[root].load(Ordering::Acquire).is_null()
    }

    pub fn alloc_root(&self, root: usize) {
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
        let root: usize = Self::section_root(section_number);
        let offset: usize = section_number & SECTION_ROOT_MASK;
        unsafe { &mut *self.0[root].load(Ordering::Acquire).add(offset) }
    }

    pub fn page_frame_number_to_page(&self, page_frame_number: usize) -> *mut Frame {
        let section_number: usize = self.page_frame_number_to_section_number(page_frame_number);
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

    pub fn page_frame_number_valid(&self, page_frame_number: usize) -> bool {
        let section_number: usize = self.page_frame_number_to_section_number(page_frame_number);
        if section_number >= MAX_SECTIONS {
            return false;
        }
        if !self.valid_section_nr(section_number) {
            return false;
        }
        self[section_number].page_frame_number_section_valid(page_frame_number)
    }

    pub fn page_frame_number_in_present_section(&self, page_frame_number: usize) -> bool {
        let section_number: usize = self.page_frame_number_to_section_number(page_frame_number);
        if section_number >= MAX_SECTIONS {
            return false;
        }
        self.present_section_nr(section_number)
    }

    pub fn valid_section_nr(&self, section_number: usize) -> bool {
        if !self.is_root_allocated(Self::section_root(section_number)) {
            return false;
        }
        self[section_number].valid_section()
    }

    pub fn online_section_nr(&self, section_number: usize) -> bool {
        if !self.is_root_allocated(Self::section_root(section_number)) {
            return false;
        }
        self[section_number].online_section()
    }

    pub fn present_section_nr(&self, section_number: usize) -> bool {
        if !self.is_root_allocated(Self::section_root(section_number)) {
            return false;
        }
        self[section_number].present_section()
    }

    pub fn get_pageblock_migratetype(&self, page_frame_number: usize) -> MigrateType {
        self.page_frame_number_to_section(page_frame_number)
            .usage
            .get_pageblock_migratetype(page_frame_number)
    }

    pub fn set_pageblock_migratetype(&self, page_frame_number: usize, migrate_type: MigrateType) {
        let section_number: usize = self.page_frame_number_to_section_number(page_frame_number);
        unsafe { self.section_mut(section_number) }
            .usage
            .set_pageblock_migratetype(page_frame_number, migrate_type);
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
        let root: usize = Self::section_root(section_number);
        let offset: usize = section_number & SECTION_ROOT_MASK;
        unsafe { &*self.0[root].load(Ordering::Acquire).add(offset) }
    }
}

pub static MEMORY_SECTION: MemorySectionTable = MemorySectionTable::new();
