use ferrum_core::linked_list::list::List;

use crate::allocator::heap::slub::free_object::FreeObject;

pub enum FrameUsage {
    Uninitialized,
    Buddy { order: usize },
    Kernel,
    PageTable,
    Slab { free: List<FreeObject> },
}

impl FrameUsage {
    pub const fn new() -> Self {
        Self::Uninitialized
    }
}
