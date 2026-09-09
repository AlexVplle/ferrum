use super::{MemoryBlockRegion, MemoryBlockRegionFlags, MEMORY_BLOCK};
use crate::physical_address::PhysicalAddress;
use crate::virtual_address::VirtualAddress;
use crate::{
    arch::fixmap::{fdt_physical_address, fdt_virtual_address},
    memory_block::MemoryBlock,
};

pub fn memory_block_init(kernel_physical_start: PhysicalAddress, kernel_size: usize) {
    let fdt_physical: PhysicalAddress = fdt_physical_address();
    let fdt_virtual: VirtualAddress = fdt_virtual_address();
    let fdt: fdt::Fdt = unsafe { fdt::Fdt::from_ptr(fdt_virtual.as_usize() as *const u8) }.unwrap();
    let fdt_base: PhysicalAddress = fdt_physical.page_base();
    let fdt_size: usize = (fdt_physical - fdt_base) + fdt.total_size();

    let memory_block: *mut MemoryBlock = &raw mut MEMORY_BLOCK;

    unsafe {
        for node in fdt.find_all_nodes("/memory") {
            let Some(mut reg) = node.reg() else {
                continue;
            };
            let Some(region) = reg.next() else {
                continue;
            };
            let Some(size) = region.size else {
                continue;
            };
            let node_id: usize = node
                .property("numa-node-id")
                .and_then(|p: fdt::node::NodeProperty<'_>| p.value.try_into().ok())
                .map(u32::from_be_bytes)
                .unwrap_or(0) as usize;
            (*memory_block).add_memory(MemoryBlockRegion {
                base: PhysicalAddress::new(region.starting_address as usize),
                size,
                flags: MemoryBlockRegionFlags::new(),
                node_id,
            });
        }

        for reservation in fdt.memory_reservations() {
            let reservation_size: usize = reservation.size();
            if reservation_size == 0 {
                continue;
            }
            (*memory_block).reserve(MemoryBlockRegion {
                base: PhysicalAddress::new(reservation.address() as usize),
                size: reservation_size,
                flags: MemoryBlockRegionFlags::new().rsrv_noinit(),
                node_id: 0,
            });
        }

        if let Some(first_memory_region) = (*memory_block).memory_regions().first() {
            let firmware_start: PhysicalAddress = first_memory_region.base;
            if kernel_physical_start.as_usize() > firmware_start.as_usize() {
                (*memory_block).reserve(MemoryBlockRegion {
                    base: firmware_start,
                    size: kernel_physical_start - firmware_start,
                    flags: MemoryBlockRegionFlags::new().rsrv_noinit(),
                    node_id: 0,
                });
            }
        }

        (*memory_block).reserve(MemoryBlockRegion {
            base: kernel_physical_start,
            size: kernel_size,
            flags: MemoryBlockRegionFlags::new().rsrv_kern(),
            node_id: 0,
        });

        (*memory_block).reserve(MemoryBlockRegion {
            base: fdt_base,
            size: fdt_size,
            flags: MemoryBlockRegionFlags::new().rsrv_noinit(),
            node_id: 0,
        });
    }
}
