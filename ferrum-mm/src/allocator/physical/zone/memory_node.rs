use super::constants::{MAX_ZONELISTS, NR_ZONES};
use super::zone::Zone;
use super::zone_list::ZoneList;
use super::zone_type::ZoneType;

pub struct MemoryNode {
    pub node_zones: [Zone; NR_ZONES],
    pub node_zonelists: [ZoneList; MAX_ZONELISTS],
    pub node_present_pages: usize,
    pub node_id: usize,
}

impl MemoryNode {
    pub fn zone_mut(&mut self, kind: ZoneType) -> &mut Zone {
        &mut self.node_zones[kind as usize]
    }

    pub const fn empty(node_id: usize) -> Self {
        const EMPTY_ZONE: Zone = Zone::empty();
        Self {
            node_zones: [EMPTY_ZONE; NR_ZONES],
            node_zonelists: [ZoneList::empty()],
            node_present_pages: 0,
            node_id,
        }
    }
}
