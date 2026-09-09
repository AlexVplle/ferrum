use super::memory_region_flags::MemoryRegionFlags;
use super::physical_address::PhysicalAddress;

#[derive(Clone, Copy)]
pub struct MemoryRegion {
    pub base: PhysicalAddress,
    pub size: u64,
    pub flags: MemoryRegionFlags,
    pub node_id: u32,
}

impl MemoryRegion {
    pub const fn empty() -> Self {
        Self {
            base: unsafe { PhysicalAddress::new_unchecked(0) },
            size: 0,
            flags: MemoryRegionFlags::new(),
            node_id: 0,
        }
    }
}
