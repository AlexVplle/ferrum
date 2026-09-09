use core::ptr::NonNull;
use core::sync::atomic::AtomicUsize;

use crate::arch::{Context, current_thread_pointer};
use crate::flags::ProcessFlags;
use crate::kernel_stack::KernelStack;
use crate::process_control_block::{INIT_TASK, ProcessControlBlock};
use ferrum_core::thread_info::ThreadInfo;
use ferrum_core::linked_list::list::linked::Linked;
use ferrum_core::linked_list::list::links::Links;

#[repr(C)]
pub struct ThreadControlBlock {
    pub thread_info: ThreadInfo,
    pub run_list: Links<ThreadControlBlock>,
    usage: AtomicUsize,
    flags: ProcessFlags,
    context: Context,
    kernel_stack: *mut KernelStack,
    process: *mut ProcessControlBlock,
}

impl ThreadControlBlock {
    pub fn current() -> &'static mut Self {
        let ptr: *mut Self = current_thread_pointer() as *mut Self;
        unsafe { &mut *ptr }
    }

    pub fn set_processor_id(&mut self, processor_id: usize) {
        self.thread_info.processor_id = processor_id;
    }
}

unsafe impl Linked<Links<ThreadControlBlock>> for ThreadControlBlock {
    fn links(ptr: NonNull<ThreadControlBlock>) -> NonNull<Links<ThreadControlBlock>> {
        unsafe { NonNull::new_unchecked(&raw mut (*ptr.as_ptr()).run_list) }
    }
}

#[unsafe(no_mangle)]
static mut INIT_KERNEL_STACK: KernelStack = KernelStack::new();

#[unsafe(no_mangle)]
pub static mut INIT_THREAD: ThreadControlBlock = ThreadControlBlock {
    thread_info: ThreadInfo::new(),
    run_list: Links::new(),
    usage: AtomicUsize::new(1),
    flags: ProcessFlags::new(),
    context: Context {
        stack_pointer: 0,
        return_address: 0,
        saved_registers: [0; 12],
        saved_fp_registers: [0; 12],
        float_csr: 0,
    },
    kernel_stack: core::ptr::null_mut(),
    process: core::ptr::null_mut(),
};

pub fn init() {
    unsafe {
        INIT_THREAD.kernel_stack = &raw mut INIT_KERNEL_STACK;
        INIT_THREAD.process = &raw mut INIT_TASK;
    }
}
