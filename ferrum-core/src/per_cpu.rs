use core::cell::UnsafeCell;
use core::mem::MaybeUninit;

pub struct PerCpu<T, const N: usize> {
    data: [UnsafeCell<MaybeUninit<T>>; N],
}

unsafe impl<T: Send, const N: usize> Sync for PerCpu<T, N> {}

impl<T, const N: usize> PerCpu<T, N> {
    pub const fn new() -> Self {
        Self {
            data: [const { UnsafeCell::new(MaybeUninit::uninit()) }; N],
        }
    }

    pub fn init(&self, cpu_id: usize, value: T) {
        unsafe { (*self.data[cpu_id].get()).write(value) };
    }

    pub fn get(&self, cpu_id: usize) -> &T {
        unsafe { (*self.data[cpu_id].get()).assume_init_ref() }
    }

    pub fn get_mut(&self, cpu_id: usize) -> &mut T {
        unsafe { (*self.data[cpu_id].get()).assume_init_mut() }
    }
}
