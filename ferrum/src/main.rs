#![no_std]
#![no_main]
#![feature(ptr_alignment_type)]

mod arch;
mod data_structures;
mod memory_management;
mod print;
mod process;
mod splash;
mod timer;

#[cfg(target_arch = "x86_64")]
mod limine;


use crate::arch::{PAGE_MASK, PAGE_SIZE};
use core::panic::PanicInfo;
use data_structures::spinlock::SpinlockGuard;
use memory_management::{
    allocator::physical::zone::allocator::ZONE_ALLOCATOR,
    early::memory_block::{MemoryBlock, MemoryBlockRegion, MemoryBlockRegionFlags, MEMORY_BLOCK},
    early::memory_map_entry::MemoryMapEntry,
    physical_address::PhysicalAddress,
};

unsafe extern "C" {
    static _kernel_start: u8;
    static _kernel_end: u8;
}

pub fn kernel_main() -> ! {
    splash::print();

    let mut buffer: [MemoryMapEntry; 8] = [MemoryMapEntry::empty(); 8];
    let count: usize = arch::memory_regions(&mut buffer).unwrap_or(0);

    printkln!("[mem] {} region(s) found", count);

    let mut total_ram: usize = 0;
    for region in &buffer[..count] {
        printkln!(
            "[mem]   base={:#x} size={:#x} node={}",
            region.base.as_usize(),
            region.size,
            region.node_id
        );
        total_ram += region.size;
    }
    printkln!("[mem] total ram: {} MiB", total_ram / (1024 * 1024));

    let mut reserved_buffer: [MemoryMapEntry; 8] = [MemoryMapEntry::empty(); 8];
    let reserved_count: usize = arch::reserved_regions(&mut reserved_buffer).unwrap_or(0);

    for reservation in &reserved_buffer[..reserved_count] {
        printkln!(
            "[mem] reserved base={:#x} size={:#x}",
            reservation.base.as_usize(),
            reservation.size
        );
    }

    let kernel_start: usize = core::ptr::addr_of!(_kernel_start) as usize;
    let kernel_end: usize = core::ptr::addr_of!(_kernel_end) as usize;
    let kernel_size: usize = kernel_end - kernel_start;

    printkln!(
        "[kernel] start={:#x} end={:#x} size={} KiB",
        kernel_start,
        kernel_end,
        kernel_size / 1024
    );

    unsafe {
        let fdt_physical: usize = arch::fdt_address() as usize;
        let fdt_virtual: usize = arch::fdt_virtual_address(fdt_physical);
        let fdt_total_size: usize = u32::from_be(*(fdt_virtual as *const u32).add(1)) as usize;
        let fdt_base: usize = fdt_physical & PAGE_MASK;

        {
            let mut memory_block: SpinlockGuard<'_, MemoryBlock> = MEMORY_BLOCK.lock();

            for entry in &buffer[..count] {
                memory_block.add_memory(MemoryBlockRegion {
                    base: entry.base,
                    size: entry.size,
                    flags: MemoryBlockRegionFlags::new(),
                    node_id: entry.node_id,
                });
            }

            for entry in &reserved_buffer[..reserved_count] {
                memory_block.reserve(MemoryBlockRegion {
                    base: entry.base,
                    size: entry.size,
                    flags: MemoryBlockRegionFlags::new().rsrv_noinit(),
                    node_id: 0,
                });
            }

            memory_block.reserve(MemoryBlockRegion {
                base: PhysicalAddress::new(kernel_start),
                size: kernel_size,
                flags: MemoryBlockRegionFlags::new().rsrv_kern(),
                node_id: 0,
            });

            memory_block.reserve(MemoryBlockRegion {
                base: PhysicalAddress::new(fdt_base),
                size: (fdt_physical - fdt_base) + fdt_total_size,
                flags: MemoryBlockRegionFlags::new().rsrv_noinit(),
                node_id: 0,
            });
        }

        arch::setup_direct_map(&buffer[..count]);
        memory_management::early::memmap::memmap_init(&buffer[..count]);

    }

    printkln!("[mm] initializing zone allocator...");
    let kernel_end: usize = core::ptr::addr_of!(_kernel_end) as usize;
    let kernel_end_physical: usize =
        kernel_end.wrapping_sub(arch::PHYSICAL_TO_VIRTUAL_OFFSET);
    let free_start: usize = (kernel_end_physical + PAGE_SIZE - 1) & PAGE_MASK;
    for region in &buffer[..count] {
        let region_end: usize = region.base.as_usize() + region.size as usize;
        if region_end <= free_start {
            continue;
        }
        let start: usize = free_start.max(region.base.as_usize());
        let num_pages: usize = (region_end - start) / PAGE_SIZE as usize;
        ZONE_ALLOCATOR.lock().add_region(PhysicalAddress::new(start), num_pages, region.node_id);
    }
    printkln!("[mm] zone allocator ready");

    timer::init();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
