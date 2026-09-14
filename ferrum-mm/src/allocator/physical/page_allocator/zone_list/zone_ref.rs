use crate::allocator::physical::page_allocator::zone_type::ZoneType;

#[derive(Clone, Copy)]
pub struct ZoneRef {
    node_id: usize,
    zone: ZoneType,
}

impl ZoneRef {
    pub const fn new(node_id: usize, zone: ZoneType) -> Self {
        Self { node_id, zone }
    }

    pub fn node_id(&self) -> usize { self.node_id }
    pub fn zone(&self) -> ZoneType { self.zone }
}
