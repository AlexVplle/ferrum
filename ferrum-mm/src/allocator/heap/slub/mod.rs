use core::ptr::NonNull;

use crate::allocator::physical::allocator::PhysicalAllocator;
use crate::allocator::physical::zone::allocator::ZONE_ALLOCATOR;
use crate::arch::{PAGE_MASK, PAGE_SIZE};
use crate::page::frame::Frame;
use crate::page::frame_usage::FrameUsage;
use crate::physical_address::PhysicalAddress;
use crate::virtual_address::VirtualAddress;
use ferrum_core::linked_list::list::List;

pub struct SlubCache {
    object_size: usize,
    partial: List<Frame>,
}

unsafe impl Send for SlubCache {}

impl SlubCache {
    pub const fn new(object_size: usize) -> Self {
        Self {
            object_size,
            partial: List::new(),
        }
    }

    fn frame_of(ptr: NonNull<u8>) -> *mut Frame {
        let page_base: usize = ptr.as_ptr() as usize & PAGE_MASK;
        VirtualAddress::new(page_base).to_page()
    }

    fn refill(&mut self) {
        let Some(phys): Option<PhysicalAddress> = ZONE_ALLOCATOR.lock().alloc_page() else {
            return;
        };
        let object_size: usize = self.object_size;
        let count: usize = PAGE_SIZE / object_size;
        let base: usize = phys.to_virtual().as_usize();
        let frame: &mut Frame = unsafe { &mut *phys.to_virtual().to_page() };

        let mut free: Option<NonNull<usize>> = None;
        for i in (0..count).rev() {
            let obj: *mut usize = (base + i * object_size) as *mut usize;
            unsafe { obj.write(free.map_or(0, |p| p.as_ptr() as usize)) };
            free = NonNull::new(obj);
        }

        frame.set_usage(FrameUsage::Slab { inuse: 0, free });
        self.partial.push_front(NonNull::from(frame));
    }

    pub fn alloc(&mut self) -> Option<NonNull<u8>> {
        if self.partial.is_empty() {
            self.refill();
        }
        let frame: &mut Frame = unsafe { self.partial.front()?.as_mut() };
        let FrameUsage::Slab { inuse, free } = frame.get_usage_mut() else {
            return None;
        };
        let head: NonNull<usize> = (*free)?;
        *free = NonNull::new(unsafe { head.as_ptr().read() } as *mut usize);
        *inuse += 1;
        if free.is_none() {
            unsafe { self.partial.remove(NonNull::from(frame)) };
        }
        Some(head.cast())
    }

    pub fn free(&mut self, ptr: NonNull<u8>) {
        let frame: &mut Frame = unsafe { &mut *Self::frame_of(ptr) };
        let FrameUsage::Slab { inuse, free } = frame.get_usage_mut() else {
            return;
        };
        let was_full: bool = free.is_none();
        let obj: *mut usize = ptr.cast::<usize>().as_ptr();
        unsafe { obj.write(free.map_or(0, |p| p.as_ptr() as usize)) };
        *free = NonNull::new(obj);
        *inuse -= 1;
        if *inuse == 0 {
            if !was_full {
                unsafe { self.partial.remove(NonNull::from(frame)) };
            }
            let page_base: usize = ptr.as_ptr() as usize & PAGE_MASK;
            ZONE_ALLOCATOR
                .lock()
                .free_page(VirtualAddress::new(page_base).to_physical());
        } else if was_full {
            self.partial.push_front(NonNull::from(frame));
        }
    }
}
