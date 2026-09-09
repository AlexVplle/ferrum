#![no_std]
#![no_main]
#![feature(ptr_alignment_type)]

mod arch;
mod elf;
mod print;
mod process;
mod splash;
mod timer;

#[cfg(target_arch = "x86_64")]
mod limine;

use core::panic::PanicInfo;
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
    memory_init();
    timer::init();

    loop {}
}

fn memory_init() {
    let kernel_start: VirtualAddress = VirtualAddress::new(&raw const (_kernel_start) as usize);
    let kernel_end: VirtualAddress = VirtualAddress::new(&raw const (_kernel_end) as usize);
    let kernel_size: usize = kernel_end - kernel_start;
    let kernel_physical_start: PhysicalAddress = kernel_start.to_kernel_physical();
    printkln!(
        "[kernel] start={} end={} size={} KiB",
        kernel_start,
        kernel_end,
        kernel_size / 1024
    );

    memory_block_init(kernel_physical_start, kernel_size);

    unsafe {
        let memory_block: &MemoryBlock = &*(&raw const MEMORY_BLOCK);

        let regions: &[MemoryBlockRegion] = memory_block.memory_regions();
        printkln!("[mem] {} region(s) found", regions.len());
        let mut total_ram: usize = 0;
        for region in regions {
            printkln!(
                "[mem]   base={} size={:#x} node={}",
                region.base,
                region.size,
                region.node_id
            );
            total_ram += region.size;
        }
        printkln!("[mem] total ram: {} MiB", total_ram / (1024 * 1024));

        for reservation in memory_block.reserved_regions() {
            printkln!(
                "[mem] reserved base={} size={:#x}",
                reservation.base,
                reservation.size
            );
        }
    }

    ferrum_mm::arch::paging::setup::setup_direct_map();
    ferrum_mm::early::memmap::memmap_init();

    unsafe {
        (*(&raw const MEMORY_BLOCK)).free_all_to_buddy(
            |base: PhysicalAddress, num_pages: usize, node_id: u32| {
                ZONE_ALLOCATOR.lock().add_region(base, num_pages, node_id);
            },
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
