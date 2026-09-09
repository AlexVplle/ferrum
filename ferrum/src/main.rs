#![no_std]
#![no_main]
#![feature(ptr_alignment_type)]

extern crate alloc;

#[global_allocator]
static ALLOCATOR: ferrum_mm::allocator::heap::kmalloc::KmallocAllocator =
    ferrum_mm::allocator::heap::kmalloc::KmallocAllocator::new();

mod arch;
mod die;
mod elf;
mod panic;
mod power;
mod smp;
mod splash;
mod system_state;
mod timer;

#[cfg(target_arch = "x86_64")]
mod limine;

use ferrum_mm::{
    PhysicalAddress, VirtualAddress,
    allocator::physical::zone::allocator::ZONE_ALLOCATOR,
    memory_block::{MEMORY_BLOCK, MemoryBlock, MemoryBlockRegion, memory_block_init},
};

unsafe extern "C" {
    static _kernel_start: u8;
    static _kernel_end: u8;
}

pub fn kernel_main() -> ! {
    splash::print();
    ferrum_core::printkln!(
        "[smp] cpu_online_mask={:#b}",
        crate::smp::CPU_ONLINE_MASK.get()
    );
    memory_init();
    timer::init();

    loop {}
}

fn memory_init() {
    let kernel_start: VirtualAddress = VirtualAddress::new(&raw const (_kernel_start) as usize);
    let kernel_end: VirtualAddress = VirtualAddress::new(&raw const (_kernel_end) as usize);
    let kernel_size: usize = kernel_end - kernel_start;
    let kernel_physical_start: PhysicalAddress = kernel_start.to_kernel_physical();
    ferrum_core::printkln!(
        "[kernel] start={} end={} size={} KiB",
        kernel_start,
        kernel_end,
        kernel_size / 1024
    );

    memory_block_init(kernel_physical_start, kernel_size);

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

    ferrum_mm::arch::paging::setup::setup_direct_map();
    ferrum_mm::early::memmap::memmap_init();

    unsafe {
        ferrum_mm::early::memtest::early_memtest(&mut *(&raw mut MEMORY_BLOCK));
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
