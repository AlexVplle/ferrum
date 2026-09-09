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

use crate::arch::{PAGE_MASK, PAGE_SIZE, PHYSICAL_TO_VIRTUAL_OFFSET};
use core::panic::PanicInfo;
use ferrum_mm::{
    allocator::physical::zone::allocator::ZONE_ALLOCATOR,
    memory_block::{MemoryBlockRegion, MemoryBlockRegionFlags, MEMORY_BLOCK},
    early::memory_map_entry::MemoryMapEntry,
    PhysicalAddress,
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

    let kernel_start: usize = &raw const(_kernel_start) as usize;
    let kernel_end: usize = &raw const(_kernel_end) as usize;
    let kernel_size: usize = kernel_end - kernel_start;
    let kernel_physical_start: usize = kernel_start.wrapping_sub(PHYSICAL_TO_VIRTUAL_OFFSET);

    printkln!(
        "[kernel] start={:#x} end={:#x} size={} KiB",
        kernel_start,
        kernel_end,
        kernel_size / 1024
    );

    unsafe {
        let fdt_physical: usize = arch::fdt_address() as usize;
        let fdt_virtual: usize = arch::fdt_virtual_address();
        let fdt_total_size: usize = u32::from_be(*(fdt_virtual as *const u32).add(1)) as usize;
        let fdt_base: usize = fdt_physical & PAGE_MASK;

        {
            let memory_block: *mut _ = &raw mut MEMORY_BLOCK;

            for entry in &buffer[..count] {
                (*memory_block).add_memory(MemoryBlockRegion {
                    base: entry.base,
                    size: entry.size,
                    flags: MemoryBlockRegionFlags::new(),
                    node_id: entry.node_id,
                });
            }

            for entry in &reserved_buffer[..reserved_count] {
                (*memory_block).reserve(MemoryBlockRegion {
                    base: entry.base,
                    size: entry.size,
                    flags: MemoryBlockRegionFlags::new().rsrv_noinit(),
                    node_id: 0,
                });
            }

            if count > 0 {
                let firmware_end: usize = kernel_physical_start;
                let firmware_start: usize = buffer[0].base.as_usize();
                if firmware_end > firmware_start {
                    (*memory_block).reserve(MemoryBlockRegion {
                        base: PhysicalAddress::new(firmware_start),
                        size: firmware_end - firmware_start,
                        flags: MemoryBlockRegionFlags::new().rsrv_noinit(),
                        node_id: 0,
                    });
                }
            }

            (*memory_block).reserve(MemoryBlockRegion {
                base: PhysicalAddress::new(kernel_physical_start),
                size: kernel_size,
                flags: MemoryBlockRegionFlags::new().rsrv_kern(),
                node_id: 0,
            });

            (*memory_block).reserve(MemoryBlockRegion {
                base: PhysicalAddress::new(fdt_base),
                size: (fdt_physical - fdt_base) + fdt_total_size,
                flags: MemoryBlockRegionFlags::new().rsrv_noinit(),
                node_id: 0,
            });
        }

        arch::setup_direct_map(&buffer[..count]);
        ferrum_mm::early::memmap::memmap_init(&buffer[..count]);

    }

    unsafe {
        (*(&raw const MEMORY_BLOCK)).free_all_to_buddy(|base: PhysicalAddress, num_pages: usize, node_id: u32| {
            ZONE_ALLOCATOR.lock().add_region(base, num_pages, node_id);
        });
    }

    timer::init();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
