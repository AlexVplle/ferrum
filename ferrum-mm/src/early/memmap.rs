use crate::memory_block::{MemoryBlockRegion, MEMORY_BLOCK};
use crate::page::frame::Frame;
use crate::page::section::constants::{PAGES_PER_SECTION, SECTIONS_PER_ROOT_BITS, SUBSECTIONS_PER_SECTION};
use crate::page::section::{MEM_SECTION, MAX_PAGE_FRAME_NUMBER, MIN_LOW_PAGE_FRAME_NUMBER, MemorySection};
use crate::physical_address::PhysicalAddress;
use core::sync::atomic::Ordering;

pub fn memmap_init() {
    let regions: &[MemoryBlockRegion] = unsafe { (*(&raw const MEMORY_BLOCK)).memory_regions() };
    for region in regions {
        let start_page_frame_number: usize = region.base.to_page_frame_number();
        let end_page_frame_number: usize =
            PhysicalAddress::new(region.base.as_usize() + region.size).to_page_frame_number();

        MIN_LOW_PAGE_FRAME_NUMBER.fetch_min(start_page_frame_number, Ordering::Relaxed);
        MAX_PAGE_FRAME_NUMBER.fetch_max(end_page_frame_number, Ordering::Relaxed);

        let start_section: usize = MEM_SECTION.section_number_for_page_frame_number(start_page_frame_number);
        let end_section: usize =
            MEM_SECTION.section_number_for_page_frame_number(end_page_frame_number.saturating_sub(1));

        for section_number in start_section..=end_section {
            let root: usize = section_number >> SECTIONS_PER_ROOT_BITS;

            if !MEM_SECTION.is_root_allocated(root) {
                MEM_SECTION.alloc_root(root);
            }

            let section_memory_map: core::ptr::NonNull<Frame> = unsafe {
                (*(&raw mut MEMORY_BLOCK))
                    .alloc(
                        PAGES_PER_SECTION * core::mem::size_of::<Frame>(),
                        core::mem::align_of::<Frame>(),
                    )
                    .expect("memmap_init: failed to allocate section memory map")
                    .to_virtual()
                    .as_non_null::<Frame>()
            };

            let section: &mut MemorySection = unsafe { MEM_SECTION.section_mut(section_number) };
            let section_base_page_frame_number: usize =
                MEM_SECTION.base_page_frame_number_for_section(section_number);
            section.set_base_page_frame_number(section_base_page_frame_number);
            section.set_memory_map(section_memory_map);
            section.flags.set_marked_present();
            section.flags.set_has_memory_map();
            section.flags.set_is_online();
            section.flags.set_is_early();

            let pages_per_subsection: usize = PAGES_PER_SECTION / SUBSECTIONS_PER_SECTION;
            for subsection in 0..SUBSECTIONS_PER_SECTION {
                let subsection_start: usize =
                    section_base_page_frame_number + subsection * pages_per_subsection;
                let subsection_end: usize = subsection_start + pages_per_subsection;
                if subsection_start < end_page_frame_number
                    && subsection_end > start_page_frame_number
                {
                    section.usage.set_subsection_present(subsection);
                }
            }
        }
    }
}
