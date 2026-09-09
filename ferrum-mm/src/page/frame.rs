use core::ptr::NonNull;
use core::sync::atomic::AtomicUsize;

use super::frame_usage::FrameUsage;
use crate::allocator::heap::slub::free_object::FreeObject;
use crate::allocator::physical::zone::ZoneType;
use ferrum_core::linked_list::list::List;
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

    pub fn set_uninitialized(&mut self) {
        self.usage = FrameUsage::Uninitialized;
    }

    pub fn set_buddy(&mut self, order: usize) {
        self.usage = FrameUsage::Buddy { order };
    }

    pub fn set_kernel(&mut self) {
        self.usage = FrameUsage::Kernel;
    }

    pub fn set_page_table(&mut self) {
        self.usage = FrameUsage::PageTable;
    }

    pub fn set_slab(&mut self, free: List<FreeObject>) {
        self.usage = FrameUsage::Slab { free };
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
