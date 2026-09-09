use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicPtr, Ordering};

use super::llist_iter::LlistIter;
use super::llist_node::LlistNode;

pub struct Llist {
    head: AtomicPtr<LlistNode>,
}

unsafe impl Send for Llist {}
unsafe impl Sync for Llist {}

impl Llist {
    pub const fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Relaxed).is_null()
    }

    /// Safety: node must not already be in a list.
    pub unsafe fn push(&self, node: NonNull<LlistNode>) {
        let mut head: *mut LlistNode = self.head.load(Ordering::Relaxed);
        loop {
            unsafe { (*node.as_ptr()).next = NonNull::new(head) };
            match self.head.compare_exchange_weak(
                head,
                node.as_ptr(),
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => return,
                Err(actual) => head = actual,
            }
        }
    }

    pub fn take(&self) -> LlistIter {
        let head: *mut LlistNode = self.head.swap(ptr::null_mut(), Ordering::Acquire);
        LlistIter::new(NonNull::new(head))
    }
}
