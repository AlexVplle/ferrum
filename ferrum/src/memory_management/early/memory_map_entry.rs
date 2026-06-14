use crate::memory_management::physical_address::PhysicalAddress;

#[derive(Clone, Copy)]
pub struct MemoryMapEntry {
    pub base: PhysicalAddress,
    pub size: usize,
    pub node_id: u32,
}

impl MemoryMapEntry {
    pub const fn empty() -> Self {
        Self {
            base: PhysicalAddress::new(0),
            size: 0,
            node_id: 0,
        }
    }
}
