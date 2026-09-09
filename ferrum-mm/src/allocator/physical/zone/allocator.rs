use super::super::allocator::PhysicalAllocator;
use super::super::buddy::BuddyAllocator;
use super::{Zone, DIRECT_MEMORY_ACCESS_ZONE_END};
use crate::arch::PAGE_SIZE;
use crate::page::frame::Frame;
use crate::page::memory_section_table::MEM_SECTION;
use crate::physical_address::PhysicalAddress;
use ferrum_core::spinlock::Spinlock;

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
        let end: PhysicalAddress = base + num_pages * PAGE_SIZE;

        if base < DIRECT_MEMORY_ACCESS_ZONE_END {
            let direct_memory_access_end: PhysicalAddress =
                PhysicalAddress::new(end.as_usize().min(DIRECT_MEMORY_ACCESS_ZONE_END));
            let direct_memory_access_pages: usize = (direct_memory_access_end - base) / PAGE_SIZE;
            if direct_memory_access_pages > 0 {
                self.direct_memory_access.init(base, direct_memory_access_pages);
                for page_frame_number in
                    base.to_page_frame_number()..direct_memory_access_end.to_page_frame_number()
                {
                    let frame: &mut Frame =
                        unsafe { &mut *MEM_SECTION.page_frame_number_to_page(page_frame_number) };
                    frame.set_zone(Zone::DirectMemoryAccess);
                    frame.set_node(node_id);
                }
            }
        }

        if end > DIRECT_MEMORY_ACCESS_ZONE_END {
            let normal_base: PhysicalAddress =
                PhysicalAddress::new(base.as_usize().max(DIRECT_MEMORY_ACCESS_ZONE_END));
            let normal_pages: usize = (end - normal_base) / PAGE_SIZE;
            if normal_pages > 0 {
                self.normal.init(normal_base, normal_pages);
                for page_frame_number in
                    normal_base.to_page_frame_number()..end.to_page_frame_number()
                {
                    let frame: &mut Frame =
                        unsafe { &mut *MEM_SECTION.page_frame_number_to_page(page_frame_number) };
                    frame.set_zone(Zone::Normal);
                    frame.set_node(node_id);
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
