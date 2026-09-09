use super::constants::{MAX_NODES, MAX_ZONELIST_ENTRIES};
use super::memory_node::MemoryNode;
use super::node_state::NodeState;
use super::node_states::NodeStates;
use super::zone_list::ZoneList;
use super::zone_ref::ZoneRef;
use super::zone_type::ZoneType;
use crate::allocator::physical::allocator::PhysicalAllocator as _;
use crate::arch::PAGE_SIZE;
use crate::page::frame::Frame;
use crate::page::memory_section_table::MEM_SECTION;
use crate::physical_address::PhysicalAddress;
use ferrum_core::spinlock::{Spinlock, SpinlockGuard};
use lock_dependency::LockClassKey;

static ZONE_NODE_KEY: LockClassKey = LockClassKey::new();

pub static ZONE_ALLOCATOR: ZoneAllocator = ZoneAllocator::new();

pub struct ZoneAllocator {
    node_data: [Spinlock<MemoryNode>; MAX_NODES],
    node_states: NodeStates,
}

unsafe impl Sync for ZoneAllocator {}

impl ZoneAllocator {
    pub const fn new() -> Self {
        const EMPTY_NODE: MemoryNode = MemoryNode::empty(0);
        const EMPTY_SLOT: Spinlock<MemoryNode> = Spinlock::new_tracked(EMPTY_NODE, &ZONE_NODE_KEY, "zone_node");
        Self {
            node_data: [EMPTY_SLOT; MAX_NODES],
            node_states: NodeStates::new(),
        }
    }

    pub fn add_region(&self, base: PhysicalAddress, num_pages: usize, node_id: usize) {
        if !self.node_states.node_is_set(node_id, NodeState::Online) {
            self.node_data[node_id].lock().id = node_id;
            self.node_states.node_set(node_id, NodeState::Online);
        }
        self.node_data[node_id].lock().add_region(base, num_pages);
    }

    pub fn build_alloc_order(&self) {
        for node_id in self.node_states.for_each_online_node() {
            let mut node: SpinlockGuard<'_, MemoryNode> = self.node_data[node_id].lock();
            let zonelist: &mut ZoneList = &mut node.alloc_order[0];
            zonelist.push(ZoneRef { node_id, zone: ZoneType::Normal });
            zonelist.push(ZoneRef { node_id, zone: ZoneType::DirectMemoryAccess });
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
            for i in 0..remote_count {
                let other_id: usize = remote_nodes[i];
                zonelist.push(ZoneRef { node_id: other_id, zone: ZoneType::Normal });
                zonelist.push(ZoneRef { node_id: other_id, zone: ZoneType::DirectMemoryAccess });
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
        let mut entries: [ZoneRef; MAX_ZONELIST_ENTRIES] = [ZoneRef {
            node_id: 0,
            zone: ZoneType::Device,
        }; MAX_ZONELIST_ENTRIES];
        let entry_count: usize = {
            let node: SpinlockGuard<'_, MemoryNode> = self.node_data[node_id].lock();
            let zonelist_entries: &[ZoneRef] = node.alloc_order[0].entries();
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
            if self.node_states.node_is_set(slot, NodeState::Online) {
                if let Some(addr) = self.node_data[slot].lock().zones[entry.zone as usize]
                    .buddy
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
        let frame: &Frame = unsafe { &*MEM_SECTION.page_frame_number_to_page(page_frame_number) };
        let slot: usize = frame.node;
        let zone_type: ZoneType = frame.zone;
        if self.node_states.node_is_set(slot, NodeState::Online) {
            self.node_data[slot].lock().zones[zone_type as usize]
                .buddy
                .free(address, size);
        }
    }
}
