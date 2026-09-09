use super::constants::{MAX_NODES, MAX_ZONELIST_ENTRIES};
use super::memory_node::MemoryNode;
use super::zone::Zone;
use super::zone_list::ZoneList;
use super::zone_ref::ZoneRef;
use super::zone_type::ZoneType;
use crate::allocator::physical::allocator::PhysicalAllocator as _;
use crate::arch::DIRECT_MEMORY_ACCESS_ZONE_END;
use crate::arch::PAGE_SIZE;
use crate::page::frame::Frame;
use crate::page::memory_section_table::MEM_SECTION;
use crate::physical_address::PhysicalAddress;
use core::sync::atomic::{AtomicUsize, Ordering};
use ferrum_core::spinlock::{Spinlock, SpinlockGuard};

pub static ZONE_ALLOCATOR: ZoneAllocator = ZoneAllocator::new();

pub struct ZoneAllocator {
    node_data: [Spinlock<MemoryNode>; MAX_NODES],
    nr_nodes: AtomicUsize,
}

unsafe impl Sync for ZoneAllocator {}

impl ZoneAllocator {
    pub const fn new() -> Self {
        const EMPTY_NODE: MemoryNode = MemoryNode::empty(0);
        const EMPTY_SLOT: Spinlock<MemoryNode> = Spinlock::new(EMPTY_NODE);
        Self {
            node_data: [EMPTY_SLOT; MAX_NODES],
            nr_nodes: AtomicUsize::new(0),
        }
    }

    pub fn add_region(&self, base: PhysicalAddress, num_pages: usize, node_id: usize) {
        let end: PhysicalAddress = base + num_pages * PAGE_SIZE;
        if self.nr_nodes.load(Ordering::Acquire) <= node_id {
            self.node_data[node_id].lock().node_id = node_id;
            self.nr_nodes.store(node_id + 1, Ordering::Release);
        }

        if let Some(direct_memory_access_zone_end) = DIRECT_MEMORY_ACCESS_ZONE_END {
            if base.as_usize() < direct_memory_access_zone_end {
                let direct_memory_access_end: PhysicalAddress =
                    PhysicalAddress::new(end.as_usize().min(direct_memory_access_zone_end));
                let direct_memory_access_pages: usize =
                    (direct_memory_access_end - base) / PAGE_SIZE;
                if direct_memory_access_pages > 0 {
                    self.init_zone(
                        node_id,
                        ZoneType::DirectMemoryAccess,
                        base,
                        direct_memory_access_pages,
                    );
                    self.tag_frames(
                        node_id,
                        ZoneType::DirectMemoryAccess,
                        base,
                        direct_memory_access_pages,
                    );
                }
            }
        }

        let normal_base: PhysicalAddress =
            if let Some(direct_memory_access_zone_end) = DIRECT_MEMORY_ACCESS_ZONE_END {
                PhysicalAddress::new(base.as_usize().max(direct_memory_access_zone_end))
            } else {
                base
            };
        let normal_pages: usize = (end - normal_base) / PAGE_SIZE;
        if normal_pages > 0 {
            self.init_zone(node_id, ZoneType::Normal, normal_base, normal_pages);
            self.tag_frames(node_id, ZoneType::Normal, normal_base, normal_pages);
        }
    }

    fn init_zone(
        &self,
        node_id: usize,
        zone_type: ZoneType,
        zone_base: PhysicalAddress,
        num_pages: usize,
    ) {
        let mut node: SpinlockGuard<'_, MemoryNode> = self.node_data[node_id].lock();
        let zone: &mut Zone = node.zone_mut(zone_type);
        zone.buddy.init(zone_base, num_pages);
        node.node_present_pages += num_pages;
    }

    fn tag_frames(
        &self,
        node_id: usize,
        zone_type: ZoneType,
        zone_base: PhysicalAddress,
        num_pages: usize,
    ) {
        let zone_end: PhysicalAddress = zone_base + num_pages * PAGE_SIZE;
        for page_frame_number in zone_base.to_page_frame_number()..zone_end.to_page_frame_number() {
            let frame: &mut Frame =
                unsafe { &mut *MEM_SECTION.page_frame_number_to_page(page_frame_number) };
            frame.set_zone(zone_type);
            frame.set_node(node_id);
        }
    }

    pub fn build_zonelists(&self) {
        let nr_nodes: usize = self.nr_nodes.load(Ordering::Acquire);
        for node_id in 0..nr_nodes {
            let mut node: SpinlockGuard<'_, MemoryNode> = self.node_data[node_id].lock();
            let zonelist: &mut ZoneList = &mut node.node_zonelists[0];
            zonelist.push(ZoneRef { node_id, zone: ZoneType::Normal });
            zonelist.push(ZoneRef { node_id, zone: ZoneType::DirectMemoryAccess });
            for other_id in 0..nr_nodes {
                if other_id != node_id {
                    zonelist.push(ZoneRef { node_id: other_id, zone: ZoneType::Normal });
                    zonelist.push(ZoneRef { node_id: other_id, zone: ZoneType::DirectMemoryAccess });
                }
            }
        }
    }

    pub fn alloc_zone_page(&self, zone_type: ZoneType) -> Option<PhysicalAddress> {
        self.alloc_zone(PAGE_SIZE, zone_type)
    }

    pub fn alloc_zone(&self, size: usize, zone_type: ZoneType) -> Option<PhysicalAddress> {
        let nr: usize = self.nr_nodes.load(Ordering::Acquire);
        if nr == 0 {
            return None;
        }
        let mut entries: [ZoneRef; MAX_ZONELIST_ENTRIES] = [ZoneRef {
            node_id: 0,
            zone: ZoneType::Device,
        }; MAX_ZONELIST_ENTRIES];
        let entry_count: usize = {
            let node: SpinlockGuard<'_, MemoryNode> = self.node_data[0].lock();
            let zonelist_entries: &[ZoneRef] = node.node_zonelists[0].entries();
            let len: usize = zonelist_entries.len();
            entries[..len].copy_from_slice(zonelist_entries);
            len
        };
        for i in 0..entry_count {
            let entry: ZoneRef = entries[i];
            if (entry.zone as usize) > (zone_type as usize) {
                continue;
            }
            let slot: usize = entry.node_id;
            if slot < nr {
                if let Some(addr) = self.node_data[slot].lock().node_zones[entry.zone as usize]
                    .buddy
                    .alloc(size, 0)
                {
                    return Some(addr);
                }
            }
        }
        None
    }

    pub fn alloc_page(&self) -> Option<PhysicalAddress> {
        self.alloc_zone_page(ZoneType::Normal)
    }

    pub fn free_page(&self, address: PhysicalAddress) {
        self.free(address, PAGE_SIZE);
    }

    pub fn free(&self, address: PhysicalAddress, size: usize) {
        let page_frame_number: usize = address.to_page_frame_number();
        let frame: &Frame = unsafe { &*MEM_SECTION.page_frame_number_to_page(page_frame_number) };
        let slot: usize = frame.node;
        let zone_type: ZoneType = frame.zone;
        let nr: usize = self.nr_nodes.load(Ordering::Acquire);
        if slot < nr {
            self.node_data[slot].lock().node_zones[zone_type as usize]
                .buddy
                .free(address, size);
        }
    }
}
