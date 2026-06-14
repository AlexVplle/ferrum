pub mod memory_block_region;
pub mod memory_block_type;

pub use memory_block_region::MemoryBlockRegion;
pub use memory_block_region::MemoryBlockRegionFlags;
use crate::data_structures::spinlock::Spinlock;
use crate::memory_management::physical_address::PhysicalAddress;
use crate::memory_management::allocator::physical::allocator::PhysicalAllocator;
use memory_block_type::MemoryBlockType;

pub struct MemoryBlock {
    memory: MemoryBlockType,
    reserved: MemoryBlockType,
    physical_memory: MemoryBlockType,
    bottom_up: bool,
    current_limit: usize,
}

impl MemoryBlock {
    pub const fn new() -> Self {
        Self {
            memory: MemoryBlockType::new(),
            reserved: MemoryBlockType::new(),
            physical_memory: MemoryBlockType::new(),
            bottom_up: false,
            current_limit: usize::MAX,
        }
    }

    pub fn set_bottom_up(&mut self, bottom_up: bool) {
        self.bottom_up = bottom_up;
    }

    pub fn set_current_limit(&mut self, limit: usize) {
        self.current_limit = limit;
    }

    pub fn add_memory(&mut self, region: MemoryBlockRegion) {
        self.memory.add(region);
        self.physical_memory.add(region);
    }

    pub fn add_physical_memory(&mut self, region: MemoryBlockRegion) {
        self.physical_memory.add(region);
    }

    pub fn reserve(&mut self, region: MemoryBlockRegion) {
        self.reserved.add(region);
    }

    fn alloc_bottom_up(&mut self, size: usize, align: usize) -> Option<PhysicalAddress> {
        for region in self.memory.regions() {
            let base: usize = region.base.as_usize();
            let end: usize = (base + region.size).min(self.current_limit);
            let mut candidate: usize = (base + align - 1) & !(align - 1);

            loop {
                if candidate + size > end {
                    break;
                }
                if let Some(conflict_end) = self.find_conflict_bottom_up(candidate, size) {
                    candidate = (conflict_end + align - 1) & !(align - 1);
                } else {
                    self.reserved.add(MemoryBlockRegion {
                        base: PhysicalAddress::new(candidate),
                        size,
                        flags: MemoryBlockRegionFlags::new(),
                        node_id: 0,
                    });
                    return Some(PhysicalAddress::new(candidate));
                }
            }
        }
        None
    }

    fn alloc_top_down(&mut self, size: usize, align: usize) -> Option<PhysicalAddress> {
        for region in self.memory.regions().iter().rev() {
            let base: usize = region.base.as_usize();
            let end: usize = (base + region.size).min(self.current_limit);

            if end < size {
                continue;
            }

            let mut candidate: usize = (end - size) & !(align - 1);

            loop {
                if candidate < base {
                    break;
                }
                if let Some(conflict_base) = self.find_conflict_top_down(candidate, size) {
                    if conflict_base < size {
                        break;
                    }
                    candidate = (conflict_base - size) & !(align - 1);
                } else {
                    self.reserved.add(MemoryBlockRegion {
                        base: PhysicalAddress::new(candidate),
                        size,
                        flags: MemoryBlockRegionFlags::new(),
                        node_id: 0,
                    });
                    return Some(PhysicalAddress::new(candidate));
                }
            }
        }
        None
    }

    fn find_conflict_bottom_up(&self, base: usize, size: usize) -> Option<usize> {
        let end: usize = base + size;
        let mut max_end: Option<usize> = None;
        for reserved in self.reserved.regions() {
            let rbase: usize = reserved.base.as_usize();
            let rend: usize = rbase + reserved.size;
            if base < rend && end > rbase {
                max_end = Some(max_end.map_or(rend, |m: usize| m.max(rend)));
            }
        }
        max_end
    }

    fn find_conflict_top_down(&self, base: usize, size: usize) -> Option<usize> {
        let end: usize = base + size;
        let mut min_base: Option<usize> = None;
        for reserved in self.reserved.regions() {
            let rbase: usize = reserved.base.as_usize();
            let rend: usize = rbase + reserved.size;
            if base < rend && end > rbase {
                min_base = Some(min_base.map_or(rbase, |m: usize| m.min(rbase)));
            }
        }
        min_base
    }
}

impl PhysicalAllocator for MemoryBlock {
    fn alloc(&mut self, size: usize, align: usize) -> Option<PhysicalAddress> {
        if self.bottom_up {
            self.alloc_bottom_up(size, align)
        } else {
            self.alloc_top_down(size, align)
        }
    }

    fn free(&mut self, _address: PhysicalAddress, _size: usize) {}
}

pub static MEMORY_BLOCK: Spinlock<MemoryBlock> = Spinlock::new(MemoryBlock::new());
