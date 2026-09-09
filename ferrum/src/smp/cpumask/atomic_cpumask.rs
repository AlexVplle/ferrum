use core::sync::atomic::{AtomicUsize, Ordering};

use super::cpumask::CpuMask;

pub struct AtomicCpuMask(AtomicUsize);

unsafe impl Send for AtomicCpuMask {}
unsafe impl Sync for AtomicCpuMask {}

impl AtomicCpuMask {
    pub const fn new() -> Self {
        Self(AtomicUsize::new(0))
    }

    pub fn set(&self, cpu: usize) {
        self.0.fetch_or(1 << cpu, Ordering::Release);
    }

    pub fn clear(&self, cpu: usize) {
        self.0.fetch_and(!(1 << cpu), Ordering::Release);
    }

    pub fn is_set(&self, cpu: usize) -> bool {
        self.0.load(Ordering::Acquire) & (1 << cpu) != 0
    }

    pub fn get(&self) -> CpuMask {
        CpuMask::from(self.0.load(Ordering::Acquire))
    }
}
