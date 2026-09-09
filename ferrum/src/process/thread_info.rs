#[repr(C)]
pub(super) struct ThreadInfo {
    pub(super) kernel_stack_pointer: usize,
    pub(super) user_stack_pointer: usize,
    pub(super) user_thread_pointer: usize,
    pub(crate) hart_id: usize,
}

pub(crate) const KERNEL_STACK_POINTER_OFFSET: usize = core::mem::offset_of!(ThreadInfo, kernel_stack_pointer);
pub(crate) const USER_STACK_POINTER_OFFSET: usize = core::mem::offset_of!(ThreadInfo, user_stack_pointer);
pub(crate) const USER_THREAD_POINTER_OFFSET: usize = core::mem::offset_of!(ThreadInfo, user_thread_pointer);

impl ThreadInfo {
    pub(super) const fn new() -> Self {
        Self { kernel_stack_pointer: 0, user_stack_pointer: 0, user_thread_pointer: 0, hart_id: 0 }
    }
}
