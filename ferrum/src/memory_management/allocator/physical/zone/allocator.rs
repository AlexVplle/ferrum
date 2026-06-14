use core::ptr::NonNull;

use super::super::buddy::BuddyAllocator;
use crate::arch::{PAGE_SIZE, PAGE_SHIFT};
use crate::data_structures::spinlock::{Spinlock, SpinlockGuard};
use crate::memory_management::early::memory_block::{MemoryBlock, MEMORY_BLOCK};
use crate::memory_management::page::memory_section::{
    physical_to_page_frame_number, set_page_node, set_page_zone,
};
use crate::memory_management::physical_address::PhysicalAddress;
use super::super::allocator::PhysicalAllocator;
use super::{DIRECT_MEMORY_ACCESS_ZONE_END, Zone};

pub static ZONE_ALLOCATOR: Spinlock<ZoneAllocator> = Spinlock::new(ZoneAllocator::empty());

pub struct ZoneAllocator {
    direct_memory_access: BuddyAllocator,
    normal: BuddyAllocator,
}

impl ZoneAllocator {
    pub const fn empty() -> Self {
        Self {
            direct_memory_access: BuddyAllocator::empty(),
            normal: BuddyAllocator::empty(),
        }
    }

    pub fn add_region(&mut self, base: PhysicalAddress, num_pages: usize, node_id: u32) {
        let start: usize = base.as_usize();
        let end: usize = start + num_pages * PAGE_SIZE;

        let mut memory_block: SpinlockGuard<'_, MemoryBlock> = MEMORY_BLOCK.lock();

        if start < DIRECT_MEMORY_ACCESS_ZONE_END && end > start {
            let dma_end: usize = end.min(DIRECT_MEMORY_ACCESS_ZONE_END);
            let dma_pages: usize = (dma_end - start) >> PAGE_SHIFT;
            if dma_pages > 0 {
                let bitmap_words: usize = BuddyAllocator::bitmap_words_needed(dma_pages);
                let bitmap: NonNull<usize> = memory_block
                    .alloc(
                        bitmap_words * core::mem::size_of::<usize>(),
                        core::mem::align_of::<usize>(),
                    )
                    .expect("zone_allocator: failed to allocate dma bitmap")
                    .to_virtual()
                    .as_non_null::<usize>();
                self.direct_memory_access.init(base, dma_pages, bitmap);
                unsafe {
                    for pfn in physical_to_page_frame_number(start)
                        ..physical_to_page_frame_number(start + dma_pages * PAGE_SIZE)
                    {
                        set_page_zone(pfn, Zone::DirectMemoryAccess);
                        set_page_node(pfn, node_id);
                    }
                }
            }
        }

        if end > DIRECT_MEMORY_ACCESS_ZONE_END {
            let normal_start: usize = start.max(DIRECT_MEMORY_ACCESS_ZONE_END);
            let normal_pages: usize = (end - normal_start) >> PAGE_SHIFT;
            if normal_pages > 0 {
                let normal_base: PhysicalAddress = PhysicalAddress::new(normal_start);
                let bitmap_words: usize = BuddyAllocator::bitmap_words_needed(normal_pages);
                let bitmap: NonNull<usize> = memory_block
                    .alloc(
                        bitmap_words * core::mem::size_of::<usize>(),
                        core::mem::align_of::<usize>(),
                    )
                    .expect("zone_allocator: failed to allocate normal bitmap")
                    .to_virtual()
                    .as_non_null::<usize>();
                self.normal.init(normal_base, normal_pages, bitmap);
                unsafe {
                    for pfn in physical_to_page_frame_number(normal_start)
                        ..physical_to_page_frame_number(normal_start + normal_pages * PAGE_SIZE)
                    {
                        set_page_zone(pfn, Zone::Normal);
                        set_page_node(pfn, node_id);
                    }
                }
            }
        }
    }

    pub fn alloc_zone_page(&mut self, zone: Zone) -> Option<PhysicalAddress> {
        self.alloc_zone(PAGE_SIZE, zone)
    }

    pub fn alloc_zone(&mut self, size: usize, zone: Zone) -> Option<PhysicalAddress> {
        match zone {
            Zone::DirectMemoryAccess => self.direct_memory_access.alloc(size, 0),
            Zone::Normal => self.normal.alloc(size, 0),
            Zone::Device => None,
        }
    }

    pub fn free_zone(&mut self, address: PhysicalAddress, size: usize, zone: Zone) {
        match zone {
            Zone::DirectMemoryAccess => self.direct_memory_access.free(address, size),
            Zone::Normal => self.normal.free(address, size),
            Zone::Device => {}
        }
    }
}

impl PhysicalAllocator for ZoneAllocator {
    fn alloc(&mut self, size: usize, align: usize) -> Option<PhysicalAddress> {
        self.normal.alloc(size, align)
    }

    fn free(&mut self, address: PhysicalAddress, size: usize) {
        if address.as_usize() < DIRECT_MEMORY_ACCESS_ZONE_END {
            self.direct_memory_access.free(address, size);
        } else {
            self.normal.free(address, size);
        }
    }
}
