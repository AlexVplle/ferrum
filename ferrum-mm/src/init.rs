use crate::PhysicalAddress;
use crate::allocator::physical::zone::allocator::ZONE_ALLOCATOR;
use crate::memory_block::{MEMORY_BLOCK, MemoryBlock, MemoryBlockRegion, memory_block_init};

pub fn memory_manager_initialization(kernel_physical_start: PhysicalAddress, kernel_size: usize) {
    memory_block_init(kernel_physical_start, kernel_size);

    #[cfg(feature = "debug")]
    unsafe {
        let memory_block: &MemoryBlock = &*(&raw const MEMORY_BLOCK);
        let regions: &[MemoryBlockRegion] = memory_block.memory_regions();
        ferrum_core::printkln!("[mem] {} region(s) found", regions.len());
        let mut total_ram: usize = 0;
        for region in regions {
            ferrum_core::printkln!(
                "[mem]   base={} size={:#x} node={}",
                region.base,
                region.size,
                region.node_id
            );
            total_ram += region.size;
        }
        ferrum_core::printkln!("[mem] total ram: {} MiB", total_ram / (1024 * 1024));
        for reservation in memory_block.reserved_regions() {
            ferrum_core::printkln!(
                "[mem] reserved base={} size={:#x}",
                reservation.base,
                reservation.size
            );
        }
    }

    crate::arch::paging::setup::setup_direct_map();
    crate::early::memmap::memmap_init();
    crate::arch::numa::numa_init();

    unsafe {
        crate::early::memtest::early_memtest(&mut *(&raw mut MEMORY_BLOCK));
    }

    unsafe {
        (*(&raw const MEMORY_BLOCK)).free_all_to_buddy(
            |base: PhysicalAddress, num_pages: usize, node_id: usize| {
                ZONE_ALLOCATOR.add_region(base, num_pages, node_id);
            },
        );
    }

    ZONE_ALLOCATOR.build_alloc_order();
}
