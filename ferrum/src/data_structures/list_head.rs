use core::marker::PhantomPinned;
use core::pin::Pin;

#[repr(C)]
pub struct ListHead {
    next: *mut ListHead,
    prev: *mut ListHead,
    _pin: PhantomPinned,
}

unsafe impl Send for ListHead {}
unsafe impl Sync for ListHead {}

impl ListHead {
    pub const fn null() -> Self {
        Self {
            next: core::ptr::null_mut(),
            prev: core::ptr::null_mut(),
            _pin: PhantomPinned,
        }
    }

    pub fn init(self: Pin<&mut Self>) {
        unsafe {
            let ptr: *mut Self = self.get_unchecked_mut();
            (*ptr).next = ptr;
            (*ptr).prev = ptr;
        }
    }

    pub fn is_initialized(&self) -> bool {
        !self.next.is_null()
    }

    pub fn is_empty(&self) -> bool {
        core::ptr::eq(self.next, self)
    }

    pub fn next_raw(&self) -> *mut ListHead {
        self.next
    }

    pub fn insert_after(self: Pin<&mut Self>, node: Pin<&mut Self>) {
        unsafe {
            let this: *mut Self = self.get_unchecked_mut();
            let node: *mut Self = node.get_unchecked_mut();
            (*node).next = (*this).next;
            (*node).prev = this;
            (*(*this).next).prev = node;
            (*this).next = node;
        }
    }

    pub fn remove(self: Pin<&mut Self>) {
        unsafe {
            let this: *mut Self = self.get_unchecked_mut();
            (*(*this).prev).next = (*this).next;
            (*(*this).next).prev = (*this).prev;
            (*this).next = this;
            (*this).prev = this;
        }
    }
}
