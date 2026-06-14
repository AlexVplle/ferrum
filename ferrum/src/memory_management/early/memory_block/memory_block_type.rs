use super::memory_block_region::constants::MAX_REGIONS;
use super::memory_block_region::MemoryBlockRegion;
use super::memory_block_region::MemoryBlockRegionFlags;
use crate::memory_management::physical_address::PhysicalAddress;

const EMPTY_REGION: MemoryBlockRegion = MemoryBlockRegion {
    base: PhysicalAddress::new(0),
    size: 0,
    flags: MemoryBlockRegionFlags::new(),
    node_id: 0,
};

pub struct MemoryBlockType {
    regions: [MemoryBlockRegion; MAX_REGIONS],
    count: usize,
}

impl MemoryBlockType {
    pub const fn new() -> Self {
        Self {
            regions: [EMPTY_REGION; MAX_REGIONS],
            count: 0,
        }
    }

    pub fn add(&mut self, region: MemoryBlockRegion) {
        if self.count < MAX_REGIONS {
            self.regions[self.count] = region;
            self.count += 1;
        }
    }

    pub fn regions(&self) -> &[MemoryBlockRegion] {
        &self.regions[..self.count]
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
