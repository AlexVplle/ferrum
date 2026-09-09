use core::sync::atomic::{AtomicU64, Ordering};

use crate::memory_management::early::memory_map_entry::MemoryMapEntry;
use crate::memory_management::physical_address::PhysicalAddress;

static FDT_ADDRESS: AtomicU64 = AtomicU64::new(0);
static HART_ID: AtomicU64 = AtomicU64::new(0);

pub fn store_fdt_address(address: u64) {
    FDT_ADDRESS.store(address, Ordering::Release);
}

pub fn fdt_address() -> u64 {
    FDT_ADDRESS.load(Ordering::Acquire)
}

pub fn store_hart_id(hartid: u64) {
    HART_ID.store(hartid, Ordering::Release);
}

pub fn hart_id() -> u64 {
    HART_ID.load(Ordering::Acquire)
}

pub fn platform_level_interrupt_controller_address() -> Option<usize> {
    let address: u64 = FDT_ADDRESS.load(Ordering::Acquire);
    let fdt: fdt::Fdt = unsafe { fdt::Fdt::from_ptr(super::fixmap::fdt_virtual_address(address as usize) as *const u8) }.ok()?;

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
    let fdt: fdt::Fdt = unsafe { fdt::Fdt::from_ptr(super::fixmap::fdt_virtual_address(address as usize) as *const u8) }.ok()?;
    let node: fdt::node::FdtNode<'_, '_> = fdt.find_node("/cpus")?;
    let property: fdt::node::NodeProperty<'_> = node.property("timebase-frequency")?;
    let bytes: [u8; 4] = property.value.try_into().ok()?;
    Some(u32::from_be_bytes(bytes) as u64)
}

pub fn reserved_regions(buffer: &mut [MemoryMapEntry]) -> Result<usize, fdt::FdtError> {
    let address: u64 = FDT_ADDRESS.load(Ordering::Acquire);
    let fdt: fdt::Fdt = unsafe { fdt::Fdt::from_ptr(super::fixmap::fdt_virtual_address(address as usize) as *const u8) }?;
    let mut count: usize = 0;

    for reservation in fdt.memory_reservations() {
        if count >= buffer.len() {
            break;
        }
        let reservation_size: usize = reservation.size();
        if reservation_size == 0 {
            continue;
        }
        buffer[count] = MemoryMapEntry {
            base: PhysicalAddress::new(reservation.address() as usize),
            size: reservation_size,
            node_id: 0,
        };
        count += 1;
    }

    Ok(count)
}

pub fn memory_regions(buffer: &mut [MemoryMapEntry]) -> Result<usize, fdt::FdtError> {
    let address: u64 = FDT_ADDRESS.load(Ordering::Acquire);
    let fdt: fdt::Fdt = unsafe { fdt::Fdt::from_ptr(super::fixmap::fdt_virtual_address(address as usize) as *const u8) }?;
    let mut count: usize = 0;

    for node in fdt.all_nodes() {
        if count >= buffer.len() {
            break;
        }
        let is_memory: bool = node.property("device_type")
            .map(|p: fdt::node::NodeProperty<'_>| {
                core::str::from_utf8(p.value)
                    .map(|s: &str| s.trim_end_matches('\0') == "memory")
                    .unwrap_or(false)
            })
            .unwrap_or(false);
        if !is_memory {
            continue;
        }
        let Some(mut reg) = node.reg() else { continue; };
        let Some(region) = reg.next() else { continue; };
        let Some(size) = region.size else { continue; };
        let node_id: u32 = node
            .property("numa-node-id")
            .and_then(|p: fdt::node::NodeProperty<'_>| p.value.try_into().ok())
            .map(u32::from_be_bytes)
            .unwrap_or(0);
        buffer[count] = MemoryMapEntry {
            base: PhysicalAddress::new(region.starting_address as usize),
            size,
            node_id,
        };
        count += 1;
    }

    Ok(count)
}
