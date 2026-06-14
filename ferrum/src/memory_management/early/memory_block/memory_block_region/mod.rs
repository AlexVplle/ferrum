pub mod constants;
pub mod memory_block_region_flags;

pub use memory_block_region_flags::MemoryBlockRegionFlags;

use crate::memory_management::physical_address::PhysicalAddress;

#[derive(Clone, Copy)]
pub struct MemoryBlockRegion {
    pub base: PhysicalAddress,
    pub size: usize,
    pub flags: MemoryBlockRegionFlags,
    pub node_id: u32,
}

impl MemoryBlockRegion {
    pub const fn empty() -> Self {
        Self {
            base: PhysicalAddress::new(0),
            size: 0,
            flags: MemoryBlockRegionFlags::new(),
            node_id: 0,
        }
    }
}
