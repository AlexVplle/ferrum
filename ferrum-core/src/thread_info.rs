pub struct ThreadInfo {
    pub kernel_stack_pointer: usize,
    pub user_stack_pointer: usize,
    pub user_thread_pointer: usize,
    pub processor_id: usize,
    pub task_id: u64,
}

pub const KERNEL_STACK_POINTER_OFFSET: usize = core::mem::offset_of!(ThreadInfo, kernel_stack_pointer);
pub const USER_STACK_POINTER_OFFSET: usize = core::mem::offset_of!(ThreadInfo, user_stack_pointer);
pub const USER_THREAD_POINTER_OFFSET: usize = core::mem::offset_of!(ThreadInfo, user_thread_pointer);
pub const PROCESSOR_ID_OFFSET: usize = core::mem::offset_of!(ThreadInfo, processor_id);
pub const TASK_ID_OFFSET: usize = core::mem::offset_of!(ThreadInfo, task_id);

impl ThreadInfo {
    pub const fn new() -> Self {
        Self {
            kernel_stack_pointer: 0,
            user_stack_pointer: 0,
            user_thread_pointer: 0,
            processor_id: 0,
            task_id: 0,
        }
    }
}
