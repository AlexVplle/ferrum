use core::sync::atomic::{AtomicU64, Ordering};

use crate::memory_management::memory_region::MemoryRegion;
use crate::memory_management::memory_region_flags::MemoryRegionFlags;
use crate::memory_management::physical_address::PhysicalAddress;

static FDT_ADDRESS: AtomicU64 = AtomicU64::new(0);

pub fn store_fdt_address(address: u64) {
    FDT_ADDRESS.store(address, Ordering::Relaxed);
}

pub fn memory_regions(buffer: &mut [MemoryRegion]) -> Result<usize, fdt::FdtError> {
    let address: u64 = FDT_ADDRESS.load(Ordering::Relaxed);
    let fdt: fdt::Fdt = unsafe { fdt::Fdt::from_ptr(address as *const u8) }?;
    let mut count: usize = 0;

    for region in fdt.memory().regions() {
        if count >= buffer.len() {
            break;
        }
        if let Some(size) = region.size {
            buffer[count] = MemoryRegion {
                base: unsafe { PhysicalAddress::new_unchecked(region.starting_address as usize) },
                size: size as u64,
                flags: MemoryRegionFlags::new(),
                node_id: 0,
            };
            count += 1;
        }
    }

    Ok(count)
}
