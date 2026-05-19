use crate::memory_management::memory_region::MemoryRegion;
use crate::memory_management::memory_region_flags::MemoryRegionFlags;
use crate::memory_management::physical_address::PhysicalAddress;

const MAX_REGIONS: usize = 128;

const EMPTY_REGION: MemoryRegion = MemoryRegion {
    base: unsafe { PhysicalAddress::new_unchecked(0) },
    size: 0,
    flags: MemoryRegionFlags::new(),
    node_id: 0,
};

pub struct MemoryBlockType {
    regions: [MemoryRegion; MAX_REGIONS],
    count: usize,
}

impl MemoryBlockType {
    pub const fn new() -> Self {
        Self {
            regions: [EMPTY_REGION; MAX_REGIONS],
            count: 0,
        }
    }

    pub fn add(&mut self, region: MemoryRegion) {
        if self.count < MAX_REGIONS {
            self.regions[self.count] = region;
            self.count += 1;
        }
    }

    pub fn regions(&self) -> &[MemoryRegion] {
        &self.regions[..self.count]
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
