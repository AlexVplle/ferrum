use core::ops::{Index, IndexMut};

use crate::arch::MAX_HARTS;

pub struct CpuToNode([usize; MAX_HARTS]);

impl CpuToNode {
    pub const fn new() -> Self {
        Self([0; MAX_HARTS])
    }
}

impl Index<usize> for CpuToNode {
    type Output = usize;
    fn index(&self, hart_id: usize) -> &usize {
        &self.0[hart_id]
    }
}

impl IndexMut<usize> for CpuToNode {
    fn index_mut(&mut self, hart_id: usize) -> &mut usize {
        &mut self.0[hart_id]
    }
}

pub static mut CPU_TO_NODE: CpuToNode = CpuToNode::new();
