pub mod inner;

use ferrum_core::spinlock::Spinlock;
use inner::PerCpuPagesInner;

pub struct PerCpuPages {
    pub inner: Spinlock<PerCpuPagesInner>,
    pub high: usize,
    pub batch: usize,
    pub free_factor: usize,
    pub expire: usize,
}

impl PerCpuPages {
    pub const fn new() -> Self {
        Self {
            inner: Spinlock::new(PerCpuPagesInner::new()),
            high: 0,
            batch: 0,
            free_factor: 1,
            expire: 0,
        }
    }
}
