use ferrum_macros::flag;

#[derive(Clone, Copy, Default)]
pub struct MemoryNodeFlags(usize);

impl MemoryNodeFlags {
    pub const fn new() -> Self {
        Self(0)
    }

    flag!(dirty, 0);
    flag!(writeback, 1);
    flag!(reclaim_locked, 2);
}
