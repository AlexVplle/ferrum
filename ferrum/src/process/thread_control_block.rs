use core::sync::atomic::AtomicU32;
use super::flags::ProcessFlags;
use super::kernel_stack::KernelStack;
use super::process_control_block::{ProcessControlBlock, INIT_TASK};
use crate::arch::Context;
use crate::data_structures::list_head::ListHead;

pub(super) struct ThreadControlBlock {
    pub run_list: ListHead,
    identifier: u64,
    usage: AtomicU32,
    flags: ProcessFlags,
    context: Context,
    kernel_stack: *mut KernelStack,
    process: *mut ProcessControlBlock,
}

#[unsafe(no_mangle)]
static mut INIT_KERNEL_STACK: KernelStack = KernelStack::new();

#[unsafe(no_mangle)]
pub(super) static mut INIT_THREAD: ThreadControlBlock = ThreadControlBlock {
    run_list: ListHead::null(),
    identifier: 0,
    usage: AtomicU32::new(1),
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

pub(super) fn init() {
    unsafe {
        INIT_THREAD.kernel_stack = &raw mut INIT_KERNEL_STACK;
        INIT_THREAD.process = &raw mut INIT_TASK;
    }
}
