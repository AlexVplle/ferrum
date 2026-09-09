pub mod free_object;

use core::ptr::NonNull;

use crate::allocator::physical::zone::allocator::ZONE_ALLOCATOR;
use crate::arch::{PAGE_MASK, PAGE_SIZE};
use crate::page::frame::Frame;
use crate::page::frame_usage::FrameUsage;
use crate::virtual_address::VirtualAddress;
use ferrum_core::linked_list::list::List;
use ferrum_core::spinlock::Spinlock;
use free_object::FreeObject;

pub struct SlubCache {
    object_size: usize,
    partial: Spinlock<List<Frame>>,
}

unsafe impl Send for SlubCache {}
unsafe impl Sync for SlubCache {}

impl SlubCache {
    pub const fn new(object_size: usize) -> Self {
        Self {
            object_size,
            partial: Spinlock::new(List::new()),
        }
    }

    fn frame_of(ptr: NonNull<u8>) -> *mut Frame {
        let page_base: usize = ptr.as_ptr() as usize & PAGE_MASK;
        VirtualAddress::new(page_base).to_page()
    }

    pub fn alloc(&self) -> Option<NonNull<FreeObject>> {
        let mut partial: ferrum_core::spinlock::SpinlockGuard<List<Frame>> = self.partial.lock();

        if partial.is_empty() {
            let physical_address = ZONE_ALLOCATOR.alloc_page()?;
            let count: usize = PAGE_SIZE / self.object_size;
            let base: usize = physical_address.to_virtual().as_usize();
            let frame: &mut Frame = unsafe { &mut *physical_address.to_virtual().to_page() };

            let mut free: List<FreeObject> = List::new();
            for i in 0..count {
                let object: NonNull<FreeObject> = unsafe {
                    NonNull::new_unchecked((base + i * self.object_size) as *mut FreeObject)
                };
                free.push_back(object);
            }
            frame.set_slab(free);
            partial.push_front(NonNull::from(frame));
        }

        let frame: &mut Frame = unsafe { partial.front()?.as_mut() };
        let FrameUsage::Slab { free } = frame.get_usage_mut() else {
            return None;
        };
        let object: NonNull<FreeObject> = free.pop_front()?;
        if free.is_empty() {
            unsafe { partial.remove(NonNull::from(frame)) };
        }
        Some(object)
    }

    pub fn free(&self, object: NonNull<FreeObject>) {
        let ptr: NonNull<u8> = object.cast();
        let frame: &mut Frame = unsafe { &mut *Self::frame_of(ptr) };
        let FrameUsage::Slab { free } = frame.get_usage_mut() else {
            return;
        };
        let was_full: bool = free.is_empty();
        free.push_front(object);
        let count: usize = PAGE_SIZE / self.object_size;
        if free.len() == count {
            if !was_full {
                let mut partial: ferrum_core::spinlock::SpinlockGuard<List<Frame>> = self.partial.lock();
                unsafe { partial.remove(NonNull::from(frame)) };
            }
            let page_base: usize = ptr.as_ptr() as usize & PAGE_MASK;
            ZONE_ALLOCATOR.free_page(VirtualAddress::new(page_base).to_physical());
        } else if was_full {
            self.partial.lock().push_front(NonNull::from(frame));
        }
    }
}
