use lock_dependency::{LockClassKey, NO_TRACK, lock_dependency_acquire, lock_dependency_release};

pub struct Spinlock<T> {
    inner: ::spinlock::Spinlock<T>,
    key: &'static LockClassKey,
    name: &'static str,
}

pub struct SpinlockGuard<'a, T> {
    inner: ::spinlock::SpinlockGuard<'a, T>,
    key: &'static LockClassKey,
}

unsafe impl<T: Send> Send for Spinlock<T> {}
unsafe impl<T: Send> Sync for Spinlock<T> {}

impl<T> Spinlock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: ::spinlock::Spinlock::new(value),
            key: &NO_TRACK,
            name: "",
        }
    }

    pub const fn new_tracked(value: T, key: &'static LockClassKey, name: &'static str) -> Self {
        Self {
            inner: ::spinlock::Spinlock::new(value),
            key,
            name,
        }
    }

    pub fn lock(&self) -> SpinlockGuard<'_, T> {
        lock_dependency_acquire(self.key, self.name, crate::arch::current_task_id() as usize);
        SpinlockGuard {
            inner: self.inner.lock(),
            key: self.key,
        }
    }

    pub fn try_lock(&self) -> Option<SpinlockGuard<'_, T>> {
        self.inner.try_lock().map(|inner: ::spinlock::SpinlockGuard<'_, T>| {
            lock_dependency_acquire(self.key, self.name, crate::arch::current_task_id() as usize);
            SpinlockGuard { inner, key: self.key }
        })
    }
}

impl<T> core::ops::Deref for SpinlockGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> core::ops::DerefMut for SpinlockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> Drop for SpinlockGuard<'_, T> {
    fn drop(&mut self) {
        lock_dependency_release(self.key, crate::arch::current_task_id() as usize);
    }
}
