use crate::allocator::physical::page_allocator::constants::MAX_NODES;

use super::constants::{LOCAL_DISTANCE, REMOTE_DISTANCE};

static mut NUMA_DISTANCE: [[u8; MAX_NODES]; MAX_NODES] = {
    let mut matrix: [[u8; MAX_NODES]; MAX_NODES] = [[REMOTE_DISTANCE; MAX_NODES]; MAX_NODES];
    let mut i: usize = 0;
    while i < MAX_NODES {
        matrix[i][i] = LOCAL_DISTANCE;
        i += 1;
    }
    matrix
};

pub fn numa_distance(from: usize, to: usize) -> u8 {
    unsafe { NUMA_DISTANCE[from][to] }
}

pub fn set(from: usize, to: usize, dist: u8) {
    if from < MAX_NODES && to < MAX_NODES {
        unsafe { NUMA_DISTANCE[from][to] = dist };
    }
}
