pub use crate::arch::{PAGE_MASK, PAGE_SHIFT, PAGE_SIZE};

pub const ZONE_SHIFT: usize = 6;
pub const ZONE_MASK: usize = 0x3 << ZONE_SHIFT;

pub const NODE_SHIFT: usize = 8;
pub const NODE_MASK: usize = 0xF << NODE_SHIFT;
