use core::sync::atomic::AtomicU32;

use super::frame_usage::FrameUsage;
use crate::memory_management::allocator::physical::zone::Zone;

pub struct Frame {
    pub zone: Zone,
    pub node: u32,
    pub usage: FrameUsage,
    pub ref_count: AtomicU32,
}

impl Frame {
    pub const fn empty() -> Self {
        Self {
            zone: Zone::Normal,
            node: 0,
            usage: FrameUsage::new(),
            ref_count: AtomicU32::new(0),
        }
    }
}
