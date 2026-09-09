mod constants;

use constants::{NUM_CLASSES, SIZE_CLASSES};
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;
use super::slub::free_object::FreeObject;
use super::slub::SlubCache;

pub struct KmallocAllocator {
    caches: [SlubCache; NUM_CLASSES],
}

impl KmallocAllocator {
    pub const fn new() -> Self {
        Self {
            caches: [
                SlubCache::new(SIZE_CLASSES[0]),
                SlubCache::new(SIZE_CLASSES[1]),
                SlubCache::new(SIZE_CLASSES[2]),
                SlubCache::new(SIZE_CLASSES[3]),
                SlubCache::new(SIZE_CLASSES[4]),
                SlubCache::new(SIZE_CLASSES[5]),
                SlubCache::new(SIZE_CLASSES[6]),
                SlubCache::new(SIZE_CLASSES[7]),
                SlubCache::new(SIZE_CLASSES[8]),
            ],
        }
    }

    fn size_class_index(size: usize) -> Option<usize> {
        let index: usize = SIZE_CLASSES.partition_point(|&class_size: &usize| class_size < size);
        (index < NUM_CLASSES).then_some(index)
    }
}

unsafe impl GlobalAlloc for KmallocAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let Some(index): Option<usize> = Self::size_class_index(layout.size()) else {
            return core::ptr::null_mut();
        };
        self.caches[index]
            .alloc()
            .map(|ptr: NonNull<FreeObject>| ptr.cast().as_ptr())
            .unwrap_or(core::ptr::null_mut())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let Some(ptr): Option<NonNull<u8>> = NonNull::new(ptr) else { return; };
        let Some(index): Option<usize> = Self::size_class_index(layout.size()) else { return; };
        self.caches[index].free(ptr.cast());
    }
}
