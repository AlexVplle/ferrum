use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

pub struct Singleton<T> {
    initialized: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for Singleton<T> {}
unsafe impl<T: Send> Sync for Singleton<T> {}

impl<T> Singleton<T> {
    pub const fn new(initial: T) -> Self {
        Self {
            initialized: AtomicBool::new(false),
            value: UnsafeCell::new(initial),
        }
    }

    pub fn init(&self, value: T) {
        unsafe { *self.value.get() = value };
        self.initialized.store(true, Ordering::Release);
    }

    pub fn get(&self) -> &T {
        assert!(self.initialized.load(Ordering::Acquire));
        unsafe { &*self.value.get() }
    }
}
