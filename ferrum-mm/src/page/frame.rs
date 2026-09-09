use core::ptr::NonNull;
use core::sync::atomic::AtomicUsize;

use super::frame_usage::FrameUsage;
use crate::allocator::physical::zone::ZoneType;
use ferrum_core::linked_list::list::linked::Linked;
use ferrum_core::linked_list::list::links::Links;

pub struct Frame {
    pub links: Links<Frame>,
    pub zone: ZoneType,
    pub node: usize,
    usage: FrameUsage,
    pub ref_count: AtomicUsize,
}

impl Frame {
    pub const fn empty() -> Self {
        Self {
            links: Links::new(),
            zone: ZoneType::Normal,
            node: 0,
            usage: FrameUsage::new(),
            ref_count: AtomicUsize::new(0),
        }
    }

    pub fn get_usage(&self) -> &FrameUsage {
        &self.usage
    }

    pub fn get_usage_mut(&mut self) -> &mut FrameUsage {
        &mut self.usage
    }

    pub fn set_usage(&mut self, usage: FrameUsage) {
        self.usage = usage;
    }

    pub fn set_zone(&mut self, zone: ZoneType) {
        self.zone = zone;
    }

    pub fn set_node(&mut self, node_id: usize) {
        self.node = node_id;
    }
}

unsafe impl Linked<Links<Frame>> for Frame {
    fn links(ptr: NonNull<Frame>) -> NonNull<Links<Frame>> {
        unsafe { NonNull::new_unchecked(&raw mut (*ptr.as_ptr()).links) }
    }
}
