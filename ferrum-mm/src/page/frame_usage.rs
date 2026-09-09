use core::ptr::NonNull;

pub enum FrameUsage {
    Uninitialized,
    Buddy { order: usize },
    Kernel,
    PageTable,
    Slab { inuse: usize, free: Option<NonNull<usize>> },
}

impl FrameUsage {
    pub const fn new() -> Self {
        Self::Uninitialized
    }
}
