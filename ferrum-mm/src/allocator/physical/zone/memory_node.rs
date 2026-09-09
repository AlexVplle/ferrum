use super::constants::{MAX_ZONELISTS, NR_ZONES};
use super::zone::Zone;
use super::zone_list::ZoneList;
use super::zone_type::ZoneType;
use crate::arch::{DIRECT_MEMORY_ACCESS_ZONE_END, PAGE_SIZE};
use crate::page::memory_section_table::MEM_SECTION;
use crate::page::frame::Frame;
use crate::physical_address::PhysicalAddress;

pub struct MemoryNode {
    pub zones: [Zone; NR_ZONES],
    pub alloc_order: [ZoneList; MAX_ZONELISTS],
    pub present_pages: usize,
    pub id: usize,
}

impl MemoryNode {
    pub fn zone_mut(&mut self, kind: ZoneType) -> &mut Zone {
        &mut self.zones[kind as usize]
    }

    pub fn init_zone(&mut self, zone_type: ZoneType, base: PhysicalAddress, num_pages: usize) {
        self.zone_mut(zone_type).init(base, num_pages);
        self.present_pages += num_pages;
        let end: PhysicalAddress = base + num_pages * PAGE_SIZE;
        for page_frame_number in base.to_page_frame_number()..end.to_page_frame_number() {
            let frame: &mut Frame =
                unsafe { &mut *MEM_SECTION.page_frame_number_to_page(page_frame_number) };
            frame.set_zone(zone_type);
            frame.set_node(self.id);
        }
    }

    pub fn add_region(&mut self, base: PhysicalAddress, num_pages: usize) {
        let end: PhysicalAddress = base + num_pages * PAGE_SIZE;

        if let Some(direct_memory_access_zone_end) = DIRECT_MEMORY_ACCESS_ZONE_END {
            if base.as_usize() < direct_memory_access_zone_end {
                let direct_memory_access_end: PhysicalAddress =
                    PhysicalAddress::new(end.as_usize().min(direct_memory_access_zone_end));
                let direct_memory_access_pages: usize =
                    (direct_memory_access_end - base) / PAGE_SIZE;
                if direct_memory_access_pages > 0 {
                    self.init_zone(ZoneType::DirectMemoryAccess, base, direct_memory_access_pages);
                }
            }
            let normal_base: PhysicalAddress =
                PhysicalAddress::new(base.as_usize().max(direct_memory_access_zone_end));
            let normal_pages: usize = (end - normal_base) / PAGE_SIZE;
            if normal_pages > 0 {
                self.init_zone(ZoneType::Normal, normal_base, normal_pages);
            }
        } else {
            self.init_zone(ZoneType::Normal, base, num_pages);
        }
    }

    pub const fn empty(node_id: usize) -> Self {
        const EMPTY_ZONE: Zone = Zone::empty();
        Self {
            zones: [EMPTY_ZONE; NR_ZONES],
            alloc_order: [ZoneList::empty()],
            present_pages: 0,
            id: node_id,
        }
    }
}
