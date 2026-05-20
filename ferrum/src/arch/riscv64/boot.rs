use core::sync::atomic::{AtomicU64, Ordering};

use crate::memory_management::memory_region::MemoryRegion;
use crate::memory_management::memory_region_flags::MemoryRegionFlags;
use crate::memory_management::physical_address::PhysicalAddress;

static FDT_ADDRESS: AtomicU64 = AtomicU64::new(0);
static HART_ID: AtomicU64 = AtomicU64::new(0);

pub fn store_fdt_address(address: u64) {
    FDT_ADDRESS.store(address, Ordering::Release);
}

pub fn store_hart_id(hartid: u64) {
    HART_ID.store(hartid, Ordering::Release);
}

pub fn hart_id() -> u64 {
    HART_ID.load(Ordering::Acquire)
}

pub fn platform_level_interrupt_controller_address() -> Option<usize> {
    let address: u64 = FDT_ADDRESS.load(Ordering::Acquire);
    let fdt: fdt::Fdt = unsafe { fdt::Fdt::from_ptr(address as *const u8) }.ok()?;

    for node in fdt.all_nodes() {
        let is_plic: bool = node.compatible()
            .map(|c: fdt::standard_nodes::Compatible<'_>| {
                c.all().any(|s: &str| s == "riscv,plic0" || s == "sifive,plic-1.0.0")
            })
            .unwrap_or(false);
        if !is_plic {
            continue;
        }
        if let Some(mut reg) = node.reg() {
            if let Some(region) = reg.next() {
                return Some(region.starting_address as usize);
            }
        }
    }

    None
}

pub fn clock_frequency() -> Option<u64> {
    let address: u64 = FDT_ADDRESS.load(Ordering::Acquire);
    let fdt: fdt::Fdt = unsafe { fdt::Fdt::from_ptr(address as *const u8) }.ok()?;
    let node: fdt::node::FdtNode<'_, '_> = fdt.find_node("/cpus")?;
    let property: fdt::node::NodeProperty<'_> = node.property("timebase-frequency")?;
    let bytes: [u8; 4] = property.value.try_into().ok()?;
    Some(u32::from_be_bytes(bytes) as u64)
}

pub fn memory_regions(buffer: &mut [MemoryRegion]) -> Result<usize, fdt::FdtError> {
    let address: u64 = FDT_ADDRESS.load(Ordering::Acquire);
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
