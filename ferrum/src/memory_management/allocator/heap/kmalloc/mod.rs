use core::ptr::NonNull;

use crate::data_structures::spinlock::Spinlock;
use super::slab::SlabCache;

const SIZE_CLASSES: [usize; 10] = [8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096];
const NUM_CLASSES: usize = SIZE_CLASSES.len();

pub static KMALLOC: Spinlock<KmallocAllocator> = Spinlock::new(KmallocAllocator::new());

pub struct KmallocAllocator {
    caches: [SlabCache; NUM_CLASSES],
}

impl KmallocAllocator {
    pub const fn new() -> Self {
        Self {
            caches: [
                SlabCache::new(SIZE_CLASSES[0]),
                SlabCache::new(SIZE_CLASSES[1]),
                SlabCache::new(SIZE_CLASSES[2]),
                SlabCache::new(SIZE_CLASSES[3]),
                SlabCache::new(SIZE_CLASSES[4]),
                SlabCache::new(SIZE_CLASSES[5]),
                SlabCache::new(SIZE_CLASSES[6]),
                SlabCache::new(SIZE_CLASSES[7]),
                SlabCache::new(SIZE_CLASSES[8]),
                SlabCache::new(SIZE_CLASSES[9]),
            ],
        }
    }

    fn size_class_index(size: usize) -> Option<usize> {
        SIZE_CLASSES.iter().position(|&class_size: &usize| class_size >= size)
    }

    pub fn alloc(&mut self, size: usize) -> Option<NonNull<u8>> {
        let index: usize = Self::size_class_index(size)?;
        self.caches[index].alloc()
    }

    pub fn free(&mut self, ptr: NonNull<u8>, size: usize) {
        let Some(index): Option<usize> = Self::size_class_index(size) else { return; };
        self.caches[index].free(ptr);
    }
}
