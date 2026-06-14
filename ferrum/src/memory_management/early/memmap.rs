use core::sync::atomic::Ordering;
use crate::arch::{PAGE_SHIFT, PAGE_SIZE};
use crate::memory_management::page::constants::{PAGES_PER_SECTION, SECTIONS_PER_ROOT};
use super::memory_block::{MemoryBlock, MEMORY_BLOCK};
use crate::data_structures::spinlock::SpinlockGuard;
use super::memory_map_entry::MemoryMapEntry;
use crate::memory_management::page::memory_section::{
    activate_section, is_root_allocated, page_frame_number_to_section_number,
    section_number_to_page_frame_number, MemorySection, MAX_PAGE_FRAME_NUMBER,
    MIN_LOW_PAGE_FRAME_NUMBER,
};
use crate::memory_management::page::frame::Frame;
use crate::memory_management::physical_address::PhysicalAddress;
use crate::memory_management::allocator::physical::allocator::PhysicalAllocator;

pub fn memmap_init(regions: &[MemoryMapEntry]) {
    unsafe {
        let mut memory_block: SpinlockGuard<'_, MemoryBlock> = MEMORY_BLOCK.lock();

        for region in regions {
            let start_page_frame_number: usize = region.base.as_usize() >> PAGE_SHIFT;
            let region_max_page_frame_number: usize =
                (region.base.as_usize() + region.size) >> PAGE_SHIFT;
            MAX_PAGE_FRAME_NUMBER.fetch_max(region_max_page_frame_number, Ordering::Relaxed);
            MIN_LOW_PAGE_FRAME_NUMBER.fetch_min(start_page_frame_number, Ordering::Relaxed);
            let end_page_frame_number: usize = start_page_frame_number + region.size / PAGE_SIZE;
            let start_section: usize = page_frame_number_to_section_number(start_page_frame_number);
            let end_section: usize =
                page_frame_number_to_section_number(end_page_frame_number.saturating_sub(1));

            for section_number in start_section..=end_section {
                let root_storage: *mut MemorySection = if !is_root_allocated(section_number) {
                    let root_size: usize =
                        SECTIONS_PER_ROOT * core::mem::size_of::<MemorySection>();
                    let root_align: usize = core::mem::align_of::<MemorySection>();
                    let address: PhysicalAddress = memory_block
                        .alloc(root_size, root_align)
                        .expect("memmap_init: failed to allocate section root");
                    let ptr: *mut MemorySection =
                        address.to_virtual().as_usize() as *mut MemorySection;
                    for i in 0..SECTIONS_PER_ROOT {
                        ptr.add(i).write(MemorySection::empty());
                    }
                    ptr
                } else {
                    core::ptr::null_mut()
                };

                let map_size: usize = PAGES_PER_SECTION * core::mem::size_of::<Frame>();
                let map_align: usize = core::mem::align_of::<Frame>();
                let mem_map_address: PhysicalAddress = memory_block
                    .alloc(map_size, map_align)
                    .expect("memmap_init: failed to allocate section_memory_map");
                let mem_map_ptr: *mut Frame =
                    mem_map_address.to_virtual().as_usize() as *mut Frame;
                for i in 0..PAGES_PER_SECTION {
                    mem_map_ptr.add(i).write(Frame::empty());
                }

                activate_section(section_number, mem_map_ptr, root_storage);

                crate::printkln!(
                    "[memmap] section {} activated: base_page_frame_number={:#x} section_memory_map={:#x}",
                    section_number,
                    section_number_to_page_frame_number(section_number),
                    mem_map_address.as_usize()
                );
            }
        }
    }
}
