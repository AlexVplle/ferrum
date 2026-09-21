use core::cell::UnsafeCell;
use super::constants::{MAX_NODES, MAX_ZONELIST_ENTRIES, NR_ZONES};
use super::zone::Zone;
use super::zone_stat_item::ZoneStatItem;
use super::memory_node::MemoryNode;
use super::node_state::NodeState;
use super::node_states::NodeStates;
use super::zone_list::ZoneList;
use super::zone_list::zone_list_type::ZoneListType;
use super::zone_list::zone_ref::ZoneRef;
use super::zone_type::ZoneType;
use crate::allocator::physical::allocator::PhysicalAllocator as _;
use crate::arch::PAGE_SIZE;
use crate::page::frame::Frame;
use crate::page::section::MEMORY_SECTION;
use crate::physical_address::PhysicalAddress;

pub static PAGE_ALLOCATOR: PageAllocator = PageAllocator::new();

pub struct PageAllocator {
    node_data: [UnsafeCell<MemoryNode>; MAX_NODES],
    node_states: NodeStates,
}

unsafe impl Sync for PageAllocator {}

impl PageAllocator {
    pub const fn new() -> Self {
        const EMPTY_NODE: MemoryNode = MemoryNode::empty(0);
        const EMPTY_CELL: UnsafeCell<MemoryNode> = UnsafeCell::new(EMPTY_NODE);
        Self {
            node_data: [EMPTY_CELL; MAX_NODES],
            node_states: NodeStates::new(),
        }
    }

    fn node(&self, id: usize) -> &MemoryNode {
        unsafe { &*self.node_data[id].get() }
    }

    fn node_mut(&self, id: usize) -> &mut MemoryNode {
        unsafe { &mut *self.node_data[id].get() }
    }

    pub fn add_region(&self, base: PhysicalAddress, num_pages: usize, node_id: usize) {
        if !self.node_states.node_is_set(node_id, NodeState::Online) {
            self.node_mut(node_id).id = node_id;
            self.node_states.node_set(node_id, NodeState::Online);
        }
        self.node_mut(node_id).add_region(base, num_pages);
    }

    pub fn build_alloc_order(&self) {
        for node_id in self.node_states.for_each_online_node() {
            let mut remote_nodes: [usize; MAX_NODES] = [0; MAX_NODES];
            let mut remote_count: usize = 0;
            for other_id in self.node_states.for_each_online_node() {
                if other_id != node_id {
                    remote_nodes[remote_count] = other_id;
                    remote_count += 1;
                }
            }
            remote_nodes[..remote_count].sort_unstable_by_key(|&other_id: &usize| {
                crate::early::numa::distance::numa_distance(node_id, other_id)
            });
            let node: &mut MemoryNode = self.node_mut(node_id);
            let fallback: &mut ZoneList = &mut node.alloc_order[ZoneListType::Fallback as usize];
            fallback.push(ZoneRef::new(node_id, ZoneType::Normal));
            fallback.push(ZoneRef::new(node_id, ZoneType::DirectMemoryAccess));
            for i in 0..remote_count {
                let other_id: usize = remote_nodes[i];
                fallback.push(ZoneRef::new(other_id, ZoneType::Normal));
                fallback.push(ZoneRef::new(other_id, ZoneType::DirectMemoryAccess));
            }
            let nofallback: &mut ZoneList = &mut node.alloc_order[ZoneListType::NoFallback as usize];
            nofallback.push(ZoneRef::new(node_id, ZoneType::Normal));
            nofallback.push(ZoneRef::new(node_id, ZoneType::DirectMemoryAccess));
        }
    }

    pub fn setup_per_zone_low_memory_reserve(&self) {
        for node_id in self.node_states.for_each_online_node() {
            let node: &mut MemoryNode = self.node_mut(node_id);
            for i in 0..NR_ZONES {
                let ratio: usize = node.zones[i].zone_type().low_memory_reserve_ratio();
                let clear: bool = ratio == 0 || !node.zones[i].populated_zone();
                let mut managed_pages: usize = 0;
                for j in (i + 1)..NR_ZONES {
                    managed_pages += node.zones[j].managed_pages();
                    node.zones[i].low_memory_reserve[j] = if clear { 0 } else { managed_pages / ratio };
                }
            }
        }
        self.calculate_total_reserve_pages();
    }

    pub fn calculate_total_reserve_pages(&self) {
        let mut total_reserve_pages: usize = 0;
        for node_id in self.node_states.for_each_online_node() {
            let node: &mut MemoryNode = self.node_mut(node_id);
            let mut reserve_pages: usize = 0;
            for i in 0..NR_ZONES {
                if !node.zones[i].populated_zone() {
                    continue;
                }
                let max_reserve: usize = node.zones[i].low_memory_reserve[i..]
                    .iter()
                    .copied()
                    .max()
                    .unwrap_or(0);
                let zone_reserve: usize = (node.zones[i].high_watermark_pages() + max_reserve)
                    .min(node.zones[i].managed_pages());
                reserve_pages += zone_reserve;
            }
            node.total_reserve_pages = reserve_pages;
            total_reserve_pages += reserve_pages;
        }
        crate::total_ram_pages::set_totalreserve_pages(total_reserve_pages);
    }

    pub fn global_zone_page_state(&self, item: ZoneStatItem) -> usize {
        let mut total: usize = 0;
        self.for_each_populated_zone(|zone: &Zone| {
            total += zone.zone_page_state(item);
        });
        total
    }

    pub fn for_each_zone<F: FnMut(&Zone)>(&self, mut function: F) {
        for node_id in self.node_states.for_each_online_node() {
            let node: &MemoryNode = self.node(node_id);
            for zone_index in 0..NR_ZONES {
                function(&node.zones[zone_index]);
            }
        }
    }

    pub fn for_each_zone_mut<F: FnMut(&mut Zone)>(&self, mut function: F) {
        for node_id in self.node_states.for_each_online_node() {
            let node: &mut MemoryNode = self.node_mut(node_id);
            for zone_index in 0..NR_ZONES {
                function(&mut node.zones[zone_index]);
            }
        }
    }

    pub fn for_each_populated_zone<F: FnMut(&Zone)>(&self, mut function: F) {
        for node_id in self.node_states.for_each_online_node() {
            let node: &MemoryNode = self.node(node_id);
            for zone_index in 0..NR_ZONES {
                if node.zones[zone_index].populated_zone() {
                    function(&node.zones[zone_index]);
                }
            }
        }
    }

    pub fn for_each_populated_zone_mut<F: FnMut(&mut Zone)>(&self, mut function: F) {
        for node_id in self.node_states.for_each_online_node() {
            let node: &mut MemoryNode = self.node_mut(node_id);
            for zone_index in 0..NR_ZONES {
                if node.zones[zone_index].populated_zone() {
                    function(&mut node.zones[zone_index]);
                }
            }
        }
    }

    pub fn alloc_zone_page(&self, zone_type: ZoneType) -> Option<PhysicalAddress> {
        self.alloc_zone(PAGE_SIZE, zone_type, self.current_node())
    }

    pub fn alloc_zone(&self, size: usize, zone_type: ZoneType, preferred_node: usize) -> Option<PhysicalAddress> {
        if self.node_states.node_is_empty(NodeState::Online) {
            return None;
        }
        let node_id: usize = if self.node_states.node_is_set(preferred_node, NodeState::Online) {
            preferred_node
        } else {
            0
        };
        let mut entries: [ZoneRef; MAX_ZONELIST_ENTRIES] = [ZoneRef::new(0, ZoneType::Device); MAX_ZONELIST_ENTRIES];
        let entry_count: usize = {
            let node: &MemoryNode = self.node(node_id);
            let zonelist_entries: &[ZoneRef] = node.alloc_order[0].entries();
            let len: usize = zonelist_entries.len();
            entries[..len].copy_from_slice(zonelist_entries);
            len
        };
        for i in 0..entry_count {
            let entry: ZoneRef = entries[i];
            if (entry.zone() as usize) > (zone_type as usize) {
                continue;
            }
            let slot: usize = entry.node_id();
            if self.node_states.node_is_set(slot, NodeState::Online) {
                if let Some(addr) = self.node(slot).zones[entry.zone() as usize]
                    .buddy.lock()
                    .alloc(size, 0)
                {
                    return Some(addr);
                }
            }
        }
        None
    }

    fn current_node(&self) -> usize {
        let processor_id: usize = ferrum_core::arch::current_processor_id();
        unsafe { crate::early::numa::CPU_TO_NODE[processor_id] }
    }

    pub fn alloc_page(&self) -> Option<PhysicalAddress> {
        self.alloc_zone(PAGE_SIZE, ZoneType::Normal, self.current_node())
    }

    pub fn alloc_pages_node(&self, node_id: usize) -> Option<PhysicalAddress> {
        self.alloc_zone(PAGE_SIZE, ZoneType::Normal, node_id)
    }

    pub fn free_page(&self, address: PhysicalAddress) {
        self.free(address, PAGE_SIZE);
    }

    pub fn free(&self, address: PhysicalAddress, size: usize) {
        let page_frame_number: usize = address.to_page_frame_number();
        let frame: &Frame = unsafe { &*MEMORY_SECTION.page_frame_number_to_page(page_frame_number) };
        let slot: usize = frame.get_node();
        let zone_type: ZoneType = frame.zone;
        if self.node_states.node_is_set(slot, NodeState::Online) {
            self.node(slot).zones[zone_type as usize]
                .buddy.lock()
                .free(address, size);
        }
    }
}
