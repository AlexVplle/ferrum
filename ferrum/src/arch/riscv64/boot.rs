use core::sync::atomic::{AtomicU64, Ordering};

use ferrum_mm::arch::fixmap::fdt_virtual_address;

static HART_ID: AtomicU64 = AtomicU64::new(0);

pub fn store_hart_id(hartid: u64) {
    HART_ID.store(hartid, Ordering::Release);
}

pub fn hart_id() -> u64 {
    HART_ID.load(Ordering::Acquire)
}

pub fn hart_count() -> usize {
    let fdt: fdt::Fdt = match unsafe {
        fdt::Fdt::from_ptr(fdt_virtual_address().as_usize() as *const u8)
    } {
        Ok(fdt) => fdt,
        Err(_) => return 1,
    };

    fdt.all_nodes()
        .filter(|node: &fdt::node::FdtNode<'_, '_>| {
            node.property("device_type")
                .and_then(|p: fdt::node::NodeProperty<'_>| core::str::from_utf8(p.value).ok())
                == Some("cpu\0")
        })
        .count()
}

pub fn platform_level_interrupt_controller_address() -> Option<usize> {
    let fdt: fdt::Fdt = unsafe { fdt::Fdt::from_ptr(fdt_virtual_address().as_usize() as *const u8) }.ok()?;

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
    let fdt: fdt::Fdt = unsafe { fdt::Fdt::from_ptr(fdt_virtual_address().as_usize() as *const u8) }.ok()?;
    let node: fdt::node::FdtNode<'_, '_> = fdt.find_node("/cpus")?;
    let property: fdt::node::NodeProperty<'_> = node.property("timebase-frequency")?;
    let bytes: [u8; 4] = property.value.try_into().ok()?;
    Some(u32::from_be_bytes(bytes) as u64)
}

