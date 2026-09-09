use core::ptr::NonNull;
use core::sync::atomic::AtomicU32;

use super::frame_usage::FrameUsage;
use ferrum_core::linked_list::linked::Linked;
use ferrum_core::linked_list::links::Links;
use crate::allocator::physical::zone::Zone;

pub struct Frame {
    pub links: Links<Frame>,
    pub zone: Zone,
    pub node: u32,
    usage: FrameUsage,
    pub ref_count: AtomicU32,
}

impl Frame {
    pub const fn empty() -> Self {
        Self {
            links: Links::new(),
            zone: Zone::Normal,
            node: 0,
            usage: FrameUsage::new(),
            ref_count: AtomicU32::new(0),
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

    pub fn set_zone(&mut self, zone: Zone) {
        self.zone = zone;
    }

    pub fn set_node(&mut self, node_id: u32) {
        self.node = node_id;
    }
}

unsafe impl Linked<Links<Frame>> for Frame {
    type Handle = NonNull<Frame>;

    fn into_ptr(handle: NonNull<Frame>) -> NonNull<Frame> { handle }
    fn from_ptr(ptr: NonNull<Frame>) -> NonNull<Frame> { ptr }
    fn links(ptr: NonNull<Frame>) -> NonNull<Links<Frame>> {
        unsafe { NonNull::new_unchecked(&raw mut (*ptr.as_ptr()).links) }
    }
}
