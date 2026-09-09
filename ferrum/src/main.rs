#![no_std]
#![no_main]
#![feature(ptr_alignment_type)]

mod arch;
mod memory_management;
mod process;

#[cfg(target_arch = "x86_64")]
mod limine;

use core::panic::PanicInfo;
use memory_management::memory_block::MemoryBlock;
use memory_management::memory_block::MEMORY_BLOCK;
use memory_management::memory_region::MemoryRegion;
use memory_management::memory_region_flags::MemoryRegionFlags;
use memory_management::physical_address::PhysicalAddress;

unsafe extern "C" {
    static _kernel_start: u8;
    static _kernel_end: u8;
}

pub fn kernel_main() -> ! {
    let mut buffer: [MemoryRegion; 8] = [MemoryRegion::empty(); 8];
    let count: usize = arch::memory_regions(&mut buffer).unwrap_or(0);

    unsafe {
        let memory_block: &mut MemoryBlock = &mut *(&raw mut MEMORY_BLOCK);

        for region in &buffer[..count] {
            memory_block.add_memory(*region);
        }

        let kernel_start: usize = core::ptr::addr_of!(_kernel_start) as usize;
        let kernel_end: usize = core::ptr::addr_of!(_kernel_end) as usize;

        memory_block.reserve(MemoryRegion {
            base: PhysicalAddress::new_unchecked(kernel_start),
            size: (kernel_end - kernel_start) as u64,
            flags: MemoryRegionFlags::new().rsrv_kern(),
            node_id: 0,
        });
    }

    unsafe { core::ptr::read_volatile(0x0 as *const u64) };

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
