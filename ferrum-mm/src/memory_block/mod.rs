mod block_type;
mod init;
pub mod region;

pub use init::memory_block_init;

pub static mut MEMORY_BLOCK: MemoryBlock = MemoryBlock::new();

pub use region::{MemoryBlockRegion, MemoryBlockRegionFlags};
use block_type::MemoryBlockType;
use crate::arch::{PAGE_MASK, PAGE_SIZE};
use crate::physical_address::PhysicalAddress;

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

    pub fn with_region(base: usize, size: usize) -> Self {
        let mut memory_block: MemoryBlock = Self::new();
        memory_block.add_memory(MemoryBlockRegion {
            base: PhysicalAddress::new(base),
            size,
            flags: MemoryBlockRegionFlags::new(),
            node_id: 0,
        });
        memory_block
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

    pub fn alloc(&mut self, size: usize, align: usize) -> Option<PhysicalAddress> {
        if self.bottom_up {
            self.alloc_bottom_up(size, align)
        } else {
            self.alloc_top_down(size, align)
        }
    }

    pub fn free(&mut self, _address: PhysicalAddress, _size: usize) {}

    pub fn memory_regions(&self) -> &[MemoryBlockRegion] {
        self.memory.regions()
    }

    pub fn reserved_regions(&self) -> &[MemoryBlockRegion] {
        self.reserved.regions()
    }

    pub fn free_all_to_buddy<F: FnMut(PhysicalAddress, usize, usize)>(&self, mut on_free_region: F) {
        for memory_region in self.memory.regions() {
            let memory_start: usize = memory_region.base.as_usize();
            let memory_end: usize = memory_start + memory_region.size;
            let mut cursor: usize = memory_start;

            while cursor < memory_end {
                let mut next_reserved_region: Option<(usize, usize)> = None;
                for reserved_region in self.reserved.regions() {
                    let reserved_start: usize = reserved_region.base.as_usize();
                    let reserved_end: usize = reserved_start + reserved_region.size;
                    if reserved_end <= cursor || reserved_start >= memory_end {
                        continue;
                    }
                    if next_reserved_region.is_none_or(|(current_start, _): (usize, usize)| reserved_start < current_start) {
                        next_reserved_region = Some((reserved_start, reserved_end));
                    }
                }

                match next_reserved_region {
                    None => {
                        let start: usize = (cursor + PAGE_SIZE - 1) & PAGE_MASK;
                        let end: usize = memory_end & PAGE_MASK;
                        if start < end {
                            on_free_region(PhysicalAddress::new(start), (end - start) / PAGE_SIZE, memory_region.node_id);
                        }
                        break;
                    }
                    Some((reserved_start, reserved_end)) => {
                        if cursor < reserved_start {
                            let start: usize = (cursor + PAGE_SIZE - 1) & PAGE_MASK;
                            let end: usize = reserved_start & PAGE_MASK;
                            if start < end {
                                on_free_region(PhysicalAddress::new(start), (end - start) / PAGE_SIZE, memory_region.node_id);
                            }
                        }
                        cursor = reserved_end.min(memory_end);
                    }
                }
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn alloc_top_down_basic() {
        let base: usize = 0x1000;
        let size: usize = 0x10000;
        let mut memory_block: MemoryBlock = MemoryBlock::with_region(base, size);
        let address: PhysicalAddress = memory_block.alloc(0x1000, 0x1000).unwrap();
        assert_eq!(address.as_usize(), base + size - 0x1000);
    }

    #[test]
    fn alloc_bottom_up_basic() {
        let base: usize = 0x1000;
        let size: usize = 0x10000;
        let mut memory_block: MemoryBlock = MemoryBlock::with_region(base, size);
        memory_block.set_bottom_up(true);
        let address: PhysicalAddress = memory_block.alloc(0x1000, 0x1000).unwrap();
        assert_eq!(address.as_usize(), base);
    }

    #[test]
    fn alloc_respects_reservation() {
        let base: usize = 0x0;
        let size: usize = 0x10000;
        let reserved_size: usize = 0x4000;
        let mut memory_block: MemoryBlock = MemoryBlock::with_region(base, size);
        memory_block.set_bottom_up(true);
        memory_block.reserve(MemoryBlockRegion {
            base: PhysicalAddress::new(base),
            size: reserved_size,
            flags: MemoryBlockRegionFlags::new(),
            node_id: 0,
        });
        let address: PhysicalAddress = memory_block.alloc(0x1000, 0x1000).unwrap();
        assert_eq!(address.as_usize(), reserved_size);
    }

    #[test]
    fn alloc_alignment() {
        let base: usize = 0x0;
        let size: usize = 0x10000;
        let align: usize = 0x2000;
        let mut memory_block: MemoryBlock = MemoryBlock::with_region(base, size);
        memory_block.set_bottom_up(true);
        let address: PhysicalAddress = memory_block.alloc(0x100, align).unwrap();
        assert_eq!(address.as_usize() % align, 0);
    }

    #[test]
    fn alloc_returns_none_when_full() {
        let base: usize = 0x0;
        let size: usize = 0x1000;
        let mut memory_block: MemoryBlock = MemoryBlock::with_region(base, size);
        memory_block.reserve(MemoryBlockRegion {
            base: PhysicalAddress::new(base),
            size,
            flags: MemoryBlockRegionFlags::new(),
            node_id: 0,
        });
        assert!(memory_block.alloc(0x1000, 0x1000).is_none());
    }

    #[test]
    fn alloc_top_down_skips_conflict() {
        let base: usize = 0x0;
        let size: usize = 0x10000;
        let reserved_base: usize = 0xF000;
        let alloc_size: usize = 0x1000;
        let mut memory_block: MemoryBlock = MemoryBlock::with_region(base, size);
        memory_block.reserve(MemoryBlockRegion {
            base: PhysicalAddress::new(reserved_base),
            size: alloc_size,
            flags: MemoryBlockRegionFlags::new(),
            node_id: 0,
        });
        let address: PhysicalAddress = memory_block.alloc(alloc_size, alloc_size).unwrap();
        assert!(address.as_usize() < reserved_base);
    }

    #[test]
    fn current_limit_respected() {
        let base: usize = 0x0;
        let size: usize = 0x10000;
        let limit: usize = 0x2000;
        let alloc_size: usize = 0x1000;
        let mut memory_block: MemoryBlock = MemoryBlock::with_region(base, size);
        memory_block.set_bottom_up(true);
        memory_block.set_current_limit(limit);
        let address: PhysicalAddress = memory_block.alloc(alloc_size, alloc_size).unwrap();
        assert!(address.as_usize() + alloc_size <= limit);
    }

    proptest! {
        #![proptest_config(proptest::test_runner::Config {
            failure_persistence: None,
            ..Default::default()
        })]

        #[test]
        fn alloc_within_region_and_aligned(
            base_pages in 1usize..256,
            region_pages in 4usize..256,
            alloc_pages in 1usize..4,
            align_log2 in 0usize..5,
            bottom_up in any::<bool>(),
        ) {
            let align: usize = 1 << align_log2;
            let base: usize = base_pages * 0x1000;
            let region_size: usize = region_pages * 0x1000;
            let alloc_size: usize = alloc_pages * 0x1000;

            let mut mb: MemoryBlock = MemoryBlock::with_region(base, region_size);
            mb.set_bottom_up(bottom_up);

            if let Some(addr) = mb.alloc(alloc_size, align) {
                let a: usize = addr.as_usize();
                prop_assert!(a >= base, "alloc below region base: {:#x} < {:#x}", a, base);
                prop_assert!(
                    a + alloc_size <= base + region_size,
                    "alloc exceeds region end: {:#x} + {:#x} > {:#x}",
                    a, alloc_size, base + region_size
                );
                prop_assert_eq!(a % align, 0, "alignment violated: {:#x} % {:#x} != 0", a, align);
            }
        }

        #[test]
        fn sequential_allocs_dont_overlap(
            base_pages in 1usize..128,
            region_pages in 8usize..256,
            alloc_pages in 1usize..4,
            bottom_up in any::<bool>(),
        ) {
            let base: usize = base_pages * 0x1000;
            let region_size: usize = region_pages * 0x1000;
            let alloc_size: usize = alloc_pages * 0x1000;
            let align: usize = 0x1000;

            let mut mb: MemoryBlock = MemoryBlock::with_region(base, region_size);
            mb.set_bottom_up(bottom_up);

            let first: Option<PhysicalAddress> = mb.alloc(alloc_size, align);
            let second: Option<PhysicalAddress> = mb.alloc(alloc_size, align);

            if let (Some(a), Some(b)) = (first, second) {
                let a: usize = a.as_usize();
                let b: usize = b.as_usize();
                let no_overlap: bool = a + alloc_size <= b || b + alloc_size <= a;
                prop_assert!(
                    no_overlap,
                    "allocations overlap: a={:#x}, b={:#x}, size={:#x}",
                    a, b, alloc_size
                );
            }
        }
    }
}
