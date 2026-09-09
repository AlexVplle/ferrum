pub mod memory_block_type;

use crate::memory_management::memory_region::MemoryRegion;
use crate::memory_management::memory_region_flags::MemoryRegionFlags;
use crate::memory_management::physical_address::PhysicalAddress;
use memory_block_type::MemoryBlockType;

pub struct MemoryBlock {
    memory: MemoryBlockType,
    reserved: MemoryBlockType,
}

impl MemoryBlock {
    pub const fn new() -> Self {
        Self {
            memory: MemoryBlockType::new(),
            reserved: MemoryBlockType::new(),
        }
    }

    pub fn add_memory(&mut self, region: MemoryRegion) {
        self.memory.add(region);
    }

    pub fn reserve(&mut self, region: MemoryRegion) {
        self.reserved.add(region);
    }

    pub fn alloc(&mut self, size: u64, align: u64) -> Option<PhysicalAddress> {
        for region in self.memory.regions() {
            let base: u64 = region.base.as_usize() as u64;
            let aligned_base: u64 = (base + align - 1) & !(align - 1);
            let end: u64 = base + region.size;

            if aligned_base + size > end {
                continue;
            }

            if self.overlaps_reserved(aligned_base, size) {
                continue;
            }

            let alloc_region: MemoryRegion = MemoryRegion {
                base: unsafe { PhysicalAddress::new_unchecked(aligned_base as usize) },
                size,
                flags: MemoryRegionFlags::new(),
                node_id: 0,
            };
            self.reserved.add(alloc_region);

            return Some(unsafe { PhysicalAddress::new_unchecked(aligned_base as usize) });
        }
        None
    }

    fn overlaps_reserved(&self, base: u64, size: u64) -> bool {
        for region in self.reserved.regions() {
            let region_base: u64 = region.base.as_usize() as u64;
            let region_end: u64 = region_base + region.size;
            let end: u64 = base + size;

            if base < region_end && end > region_base {
                return true;
            }
        }
        false
    }
}

pub static mut MEMORY_BLOCK: MemoryBlock = MemoryBlock::new();
