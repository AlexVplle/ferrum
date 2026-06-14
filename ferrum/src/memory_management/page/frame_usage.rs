pub enum FrameUsage {
    Buddy { order: usize },
    Kernel,
    PageTable,
    Slab,
}

impl FrameUsage {
    pub const fn new() -> Self {
        Self::Buddy { order: 0 }
    }
}
