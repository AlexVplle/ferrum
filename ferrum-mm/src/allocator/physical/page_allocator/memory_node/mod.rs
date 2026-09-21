pub mod flags;
pub mod lru_list;
pub mod lru_vector;
pub mod node_stat_item;

use super::constants::{MAX_ZONELISTS, NR_ZONES};
use super::zone::Zone;
use super::zone_list::ZoneList;
use super::zone_type::ZoneType;
use crate::arch::{DIRECT_MEMORY_ACCESS_ZONE_END, PAGE_SIZE};
use crate::page::section::MEMORY_SECTION;
use crate::page::frame::Frame;
use crate::physical_address::PhysicalAddress;
use flags::MemoryNodeFlags;
use lru_vector::LruVector;

pub struct MemoryNode {
    pub zones: [Zone; NR_ZONES],
    pub alloc_order: [ZoneList; MAX_ZONELISTS],
    pub start_page_frame_number: usize,
    spanned_pages: usize,
    present_pages: usize,
    pub nr_zones: usize,
    pub total_reserve_pages: usize,
    pub flags: MemoryNodeFlags,
    pub id: usize,
    pub lru_vector: LruVector,
}

impl MemoryNode {
    pub fn spanned_pages(&self) -> usize {
        self.spanned_pages
    }

    pub fn present_pages(&self) -> usize {
        self.present_pages
    }

    pub fn end_page_frame_number(&self) -> usize {
        self.start_page_frame_number + self.spanned_pages
    }

    pub fn memory_control_group_lruvector(&mut self) -> &mut LruVector {
        &mut self.lru_vector
    }

    pub fn zone_mut(&mut self, kind: ZoneType) -> &mut Zone {
        &mut self.zones[kind as usize]
    }

    pub fn init_zone(&mut self, zone_type: ZoneType, base: PhysicalAddress, num_pages: usize) {
        let node_id: usize = self.id;
        self.zone_mut(zone_type).init(zone_type, node_id, base, num_pages);
        self.present_pages += num_pages;
        self.nr_zones += 1;
        let end: PhysicalAddress = base + num_pages * PAGE_SIZE;
        for page_frame_number in base.to_page_frame_number()..end.to_page_frame_number() {
            let frame: &mut Frame =
                unsafe { &mut *MEMORY_SECTION.page_frame_number_to_page(page_frame_number) };
            frame.set_zone(zone_type);
            frame.set_node(self.id);
        }
    }

    pub fn add_region(&mut self, base: PhysicalAddress, num_pages: usize) {
        let end: PhysicalAddress = base + num_pages * PAGE_SIZE;
        let start_page_frame_number: usize = base.to_page_frame_number();
        let end_page_frame_number: usize = end.to_page_frame_number();
        if self.spanned_pages == 0 {
            self.start_page_frame_number = start_page_frame_number;
        } else {
            self.start_page_frame_number = self.start_page_frame_number.min(start_page_frame_number);
        }
        let current_end_page_frame_number: usize = self.start_page_frame_number + self.spanned_pages;
        self.spanned_pages = end_page_frame_number.max(current_end_page_frame_number) - self.start_page_frame_number;

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
            alloc_order: [ZoneList::empty(), ZoneList::empty()],
            start_page_frame_number: 0,
            spanned_pages: 0,
            present_pages: 0,
            nr_zones: 0,
            total_reserve_pages: 0,
            flags: MemoryNodeFlags::new(),
            id: node_id,
            lru_vector: LruVector::new(),
        }
    }
}
