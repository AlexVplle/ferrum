use core::sync::atomic::{AtomicUsize, Ordering};

use crate::bitmask_iter::BitmaskIter;

pub struct AtomicBitmask(AtomicUsize);

unsafe impl Send for AtomicBitmask {}
unsafe impl Sync for AtomicBitmask {}

impl AtomicBitmask {
    pub const fn new() -> Self {
        Self(AtomicUsize::new(0))
    }

    pub fn set(&self, nr: usize) {
        self.0.fetch_or(1 << nr, Ordering::Release);
    }

    pub fn clear(&self, nr: usize) {
        self.0.fetch_and(!(1 << nr), Ordering::Release);
    }

    pub fn change(&self, nr: usize) {
        self.0.fetch_xor(1 << nr, Ordering::Release);
    }

    pub fn test(&self, nr: usize) -> bool {
        self.0.load(Ordering::Acquire) & (1 << nr) != 0
    }

    pub fn test_and_set(&self, nr: usize) -> bool {
        self.0.fetch_or(1 << nr, Ordering::AcqRel) & (1 << nr) != 0
    }

    pub fn test_and_clear(&self, nr: usize) -> bool {
        self.0.fetch_and(!(1 << nr), Ordering::AcqRel) & (1 << nr) != 0
    }

    pub fn get(&self) -> usize {
        self.0.load(Ordering::Acquire)
    }

    pub fn store(&self, mask: usize) {
        self.0.store(mask, Ordering::Release);
    }

    pub fn set_all(&self) {
        self.0.store(usize::MAX, Ordering::Release);
    }

    pub fn is_empty(&self) -> bool {
        self.0.load(Ordering::Acquire) == 0
    }

    pub fn find_first_set(&self) -> Option<usize> {
        let word: usize = self.0.load(Ordering::Acquire);
        if word == 0 { None } else { Some(word.trailing_zeros() as usize) }
    }

    pub fn find_first_zero(&self) -> Option<usize> {
        let word: usize = !self.0.load(Ordering::Acquire);
        if word == 0 { None } else { Some(word.trailing_zeros() as usize) }
    }

    pub fn count_set(&self) -> usize {
        self.0.load(Ordering::Acquire).count_ones() as usize
    }

    pub fn iter(&self) -> BitmaskIter {
        BitmaskIter::new(self.0.load(Ordering::Acquire))
    }
}
