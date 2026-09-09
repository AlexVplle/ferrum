#[cfg(loom)]
use loom::sync::atomic::AtomicBool;
#[cfg(not(loom))]
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

pub struct Spinlock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct SpinlockGuard<'a, T> {
    lock: &'a Spinlock<T>,
}

unsafe impl<T: Send> Send for Spinlock<T> {}
unsafe impl<T: Send> Sync for Spinlock<T> {}

impl<T> Spinlock<T> {
    #[cfg(not(loom))]
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    #[cfg(loom)]
    pub fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> SpinlockGuard<'_, T> {
        loop {
            if self
                .locked
                .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }
            while self.locked.load(Ordering::Relaxed) {
                #[cfg(loom)]
                loom::hint::spin_loop();
                #[cfg(not(loom))]
                core::hint::spin_loop();
            }
        }
        SpinlockGuard { lock: self }
    }
}

impl<T> Drop for SpinlockGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

impl<T> Deref for SpinlockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for SpinlockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

#[cfg(loom)]
mod loom_tests {
    use super::*;
    use loom::sync::Arc;

    #[test]
    fn lock_and_read() {
        loom::model(|| {
            let spinlock: Spinlock<usize> = Spinlock::new(42);
            let guard: SpinlockGuard<'_, usize> = spinlock.lock();
            assert_eq!(*guard, 42);
        });
    }

    #[test]
    fn lock_and_write() {
        loom::model(|| {
            let spinlock: Spinlock<usize> = Spinlock::new(0);
            {
                let mut guard: SpinlockGuard<'_, usize> = spinlock.lock();
                *guard = 99;
            }
            let guard: SpinlockGuard<'_, usize> = spinlock.lock();
            assert_eq!(*guard, 99);
        });
    }

    #[test]
    fn unlocks_on_drop() {
        loom::model(|| {
            let spinlock: Spinlock<usize> = Spinlock::new(0);
            {
                let _guard: SpinlockGuard<'_, usize> = spinlock.lock();
            }
            assert!(!spinlock.locked.load(Ordering::Relaxed));
        });
    }

    #[test]
    fn concurrent_increment() {
        loom::model(|| {
            let spinlock: Arc<Spinlock<usize>> = Arc::new(Spinlock::new(0));
            let s2: Arc<Spinlock<usize>> = Arc::clone(&spinlock);

            let t: loom::thread::JoinHandle<()> = loom::thread::spawn(move || {
                *s2.lock() += 1;
            });

            *spinlock.lock() += 1;
            t.join().unwrap();

            assert_eq!(*spinlock.lock(), 2);
        });
    }
}
