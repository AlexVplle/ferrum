use crate::constants::KERNEL_STACK_SIZE;

#[repr(C, align(4096))]
pub struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

impl KernelStack {
    pub const fn new() -> Self {
        Self { data: [0; KERNEL_STACK_SIZE] }
    }
}
