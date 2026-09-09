use crate::atomic_bitmask::AtomicBitmask;
use crate::bitmask_iter::BitmaskIter;

pub struct StateMap<const N: usize>([AtomicBitmask; N]);

impl<const N: usize> StateMap<N> {
    pub const fn new() -> Self {
        Self([const { AtomicBitmask::new() }; N])
    }

    pub fn set(&self, id: usize, state: usize) {
        self.0[state].set(id);
    }

    pub fn clear(&self, id: usize, state: usize) {
        self.0[state].clear(id);
    }

    pub fn is_set(&self, id: usize, state: usize) -> bool {
        self.0[state].test(id)
    }

    pub fn iter(&self, state: usize) -> BitmaskIter {
        self.0[state].iter()
    }

    pub fn is_empty(&self, state: usize) -> bool {
        self.0[state].is_empty()
    }

    pub fn set_all(&self, state: usize) {
        self.0[state].set_all();
    }

    pub fn get_mask(&self, state: usize) -> usize {
        self.0[state].get()
    }
}
