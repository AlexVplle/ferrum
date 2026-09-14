use crate::memory_block::{MemoryBlockRegion, MEMORY_BLOCK};
use crate::page::frame::Frame;
use crate::page::section::constants::{PAGES_PER_SECTION, PAGES_PER_SUBSECTION, SUBSECTIONS_PER_SECTION};
use crate::page::section::memory_section_table::MemorySectionTable;
use crate::page::section::{MEMORY_SECTION, MAX_PAGE_FRAME_NUMBER, MIN_LOW_PAGE_FRAME_NUMBER, MemorySection};
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

        let start_section: usize = MEMORY_SECTION.page_frame_number_to_section_number(start_page_frame_number);
        let end_section: usize =
            MEMORY_SECTION.page_frame_number_to_section_number(end_page_frame_number.saturating_sub(1));

        for section_number in start_section..=end_section {
            let root: usize = MemorySectionTable::section_root(section_number);

            if !MEMORY_SECTION.is_root_allocated(root) {
                MEMORY_SECTION.alloc_root(root);
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

            let section: &mut MemorySection = unsafe { MEMORY_SECTION.section_mut(section_number) };
            let section_base_page_frame_number: usize =
                MEMORY_SECTION.base_page_frame_number_for_section(section_number);
            section.set_base_page_frame_number(section_base_page_frame_number);
            section.set_memory_map(section_memory_map);
            section.flags.set_marked_present();
            section.flags.set_has_memory_map();
            section.flags.set_online();
            section.flags.set_early();

            for subsection in 0..SUBSECTIONS_PER_SECTION {
                let subsection_start: usize =
                    section_base_page_frame_number + subsection * PAGES_PER_SUBSECTION;
                let subsection_end: usize = subsection_start + PAGES_PER_SUBSECTION;
                if subsection_start < end_page_frame_number
                    && subsection_end > start_page_frame_number
                {
                    section.usage.set_subsection_present(subsection);
                }
            }
        }
    }
}
