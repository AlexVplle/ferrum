use core::mem::size_of;

use crate::allocator::physical::zone::constants::MAX_NODES;
use crate::arch::{fixmap::fdt_virtual_address, MAX_HARTS};
use crate::early::numa::constants::DISTANCE_MATRIX_ENTRY_SIZE;
use crate::early::numa::{CPU_TO_NODE, distance};
use crate::virtual_address::VirtualAddress;

fn read_u32(data: &[u8], offset: usize) -> usize {
    u32::from_be_bytes(
        data[offset..offset + size_of::<u32>()]
            .try_into()
            .unwrap_or([0; 4]),
    ) as usize
}

fn parse_distance_entry(data: &[u8], entry_index: usize) {
    let offset: usize = entry_index * DISTANCE_MATRIX_ENTRY_SIZE;
    let from: usize = read_u32(data, offset);
    let to: usize = read_u32(data, offset + size_of::<u32>());
    let dist: u8 = read_u32(data, offset + 2 * size_of::<u32>()) as u8;
    distance::set(from, to, dist);
}

pub fn numa_init() {
    let fdt_virtual: VirtualAddress = fdt_virtual_address();
    let fdt: fdt::Fdt =
        unsafe { fdt::Fdt::from_ptr(fdt_virtual.as_usize() as *const u8) }.unwrap();

    for cpu in fdt.find_all_nodes("/cpus/cpu") {
        let Some(reg) = cpu.property("reg") else {
            continue;
        };
        let hart_id: usize = reg
            .value
            .try_into()
            .map(u32::from_be_bytes)
            .unwrap_or(0) as usize;
        if hart_id >= MAX_HARTS {
            continue;
        }
        let node_id: usize = cpu
            .property("numa-node-id")
            .and_then(|node_property: fdt::node::NodeProperty<'_>| {
                node_property.value.try_into().ok()
            })
            .map(u32::from_be_bytes)
            .unwrap_or(0) as usize;
        if node_id < MAX_NODES {
            unsafe { CPU_TO_NODE[hart_id] = node_id };
        }
    }

    if let Some(distance_map) = fdt.find_node("/distance-map") {
        if let Some(node_property) = distance_map.property("distance-matrix") {
            let data: &[u8] = node_property.value;
            let entries: usize = data.len() / DISTANCE_MATRIX_ENTRY_SIZE;
            for i in 0..entries {
                parse_distance_entry(data, i);
            }
        }
    }
}
