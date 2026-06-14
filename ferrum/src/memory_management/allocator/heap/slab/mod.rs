use core::ptr::NonNull;

use crate::arch::PAGE_SIZE;
use crate::memory_management::physical_address::PhysicalAddress;
use crate::memory_management::allocator::physical::allocator::PhysicalAllocator;
use crate::memory_management::allocator::physical::zone::allocator::ZONE_ALLOCATOR;

pub struct SlabCache {
    object_size: usize,
    free_list: Option<NonNull<usize>>,
}

unsafe impl Send for SlabCache {}

impl SlabCache {
    pub const fn new(object_size: usize) -> Self {
        Self {
            object_size,
            free_list: None,
        }
    }

    fn refill(&mut self) {
        let Some(page): Option<PhysicalAddress> = ZONE_ALLOCATOR.lock().alloc_page() else { return; };
        let base: usize = page.to_virtual().as_usize();
        let count: usize = PAGE_SIZE / self.object_size;
        for i in (0..count).rev() {
            let object: *mut usize = (base + i * self.object_size) as *mut usize;
            unsafe {
                object.write(self.free_list.map_or(0usize, |p: NonNull<usize>| p.as_ptr() as usize));
            }
            self.free_list = NonNull::new(object);
        }
    }

    pub fn alloc(&mut self) -> Option<NonNull<u8>> {
        if self.free_list.is_none() {
            self.refill();
        }
        let head: NonNull<usize> = self.free_list?;
        let next: usize = unsafe { head.as_ptr().read() };
        self.free_list = NonNull::new(next as *mut usize);
        Some(head.cast::<u8>())
    }

    pub fn free(&mut self, ptr: NonNull<u8>) {
        let object: *mut usize = ptr.cast::<usize>().as_ptr();
        unsafe {
            object.write(self.free_list.map_or(0usize, |p: NonNull<usize>| p.as_ptr() as usize));
        }
        self.free_list = NonNull::new(object);
    }
}
