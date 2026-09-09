use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

use super::constants::{
    NR_SECTION_ROOTS, PAGES_PER_SECTION, PAGE_FRAME_NUMBER_SECTION_SHIFT, SECTIONS_PER_ROOT,
    SECTIONS_PER_ROOT_BITS,
};
use super::frame::Frame;
use crate::memory_management::allocator::physical::zone::Zone;

pub static MAX_PAGE_FRAME_NUMBER: AtomicUsize = AtomicUsize::new(0);
pub static MIN_LOW_PAGE_FRAME_NUMBER: AtomicUsize = AtomicUsize::new(usize::MAX);

static MEM_SECTION: [AtomicPtr<MemorySection>; NR_SECTION_ROOTS] =
    [const { AtomicPtr::new(core::ptr::null_mut()) }; NR_SECTION_ROOTS];

pub struct MemorySection {
    pub section_memory_map: *mut Frame,
    pub base_page_frame_number: usize,
}

impl MemorySection {
    pub const fn empty() -> Self {
        Self {
            section_memory_map: core::ptr::null_mut(),
            base_page_frame_number: 0,
        }
    }

    pub fn is_present(&self) -> bool {
        !self.section_memory_map.is_null()
    }
}

pub fn page_frame_number_to_section_number(page_frame_number: usize) -> usize {
    page_frame_number >> PAGE_FRAME_NUMBER_SECTION_SHIFT
}

pub fn section_number_to_page_frame_number(section_number: usize) -> usize {
    section_number << PAGE_FRAME_NUMBER_SECTION_SHIFT
}

pub fn is_root_allocated(section_number: usize) -> bool {
    let root: usize = section_number >> SECTIONS_PER_ROOT_BITS;
    !MEM_SECTION[root].load(Ordering::Acquire).is_null()
}

pub unsafe fn activate_section(
    section_number: usize,
    section_memory_map: *mut Frame,
    root_storage: *mut MemorySection,
) {
    let root: usize = section_number >> SECTIONS_PER_ROOT_BITS;
    let offset: usize = section_number & (SECTIONS_PER_ROOT - 1);
    unsafe {
        if MEM_SECTION[root].load(Ordering::Acquire).is_null() {
            MEM_SECTION[root].store(root_storage, Ordering::Release);
        }
        let section: &mut MemorySection =
            &mut *MEM_SECTION[root].load(Ordering::Acquire).add(offset);
        section.section_memory_map = section_memory_map;
        section.base_page_frame_number = section_number_to_page_frame_number(section_number);
    }
}

pub unsafe fn page_frame_number_to_page(page_frame_number: usize) -> *mut Frame {
    let section_number: usize = page_frame_number_to_section_number(page_frame_number);
    let root: usize = section_number >> SECTIONS_PER_ROOT_BITS;
    let offset: usize = section_number & (SECTIONS_PER_ROOT - 1);
    unsafe {
        let section: &MemorySection =
            &*MEM_SECTION[root].load(Ordering::Acquire).add(offset);
        let page_frame_number_in_section: usize =
            page_frame_number - section.base_page_frame_number;
        section.section_memory_map.add(page_frame_number_in_section)
    }
}

pub unsafe fn page_to_page_frame_number(page: *const Frame, section_number: usize) -> usize {
    let root: usize = section_number >> SECTIONS_PER_ROOT_BITS;
    let offset: usize = section_number & (SECTIONS_PER_ROOT - 1);
    unsafe {
        let section: &MemorySection =
            &*MEM_SECTION[root].load(Ordering::Acquire).add(offset);
        section.base_page_frame_number
            + (page as usize - section.section_memory_map as usize)
                / core::mem::size_of::<Frame>()
    }
}

pub unsafe fn page_frame_number_to_physical(page_frame_number: usize) -> usize {
    page_frame_number << super::constants::PAGE_SHIFT
}

pub fn physical_to_page_frame_number(physical_address: usize) -> usize {
    physical_address >> super::constants::PAGE_SHIFT
}

pub unsafe fn set_page_zone(page_frame_number: usize, zone: Zone) {
    unsafe {
        let frame: &mut Frame = &mut *page_frame_number_to_page(page_frame_number);
        frame.zone = zone;
    }
}

pub unsafe fn set_page_node(page_frame_number: usize, node_id: u32) {
    unsafe {
        let frame: &mut Frame = &mut *page_frame_number_to_page(page_frame_number);
        frame.node = node_id;
    }
}
