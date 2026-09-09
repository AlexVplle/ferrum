use super::memory_descriptor::MemoryDescriptor;
use super::state::ProcessState;
use crate::data_structures::list_head::ListHead;

pub(super) struct ProcessControlBlock {
    identifier: u64,
    state: ProcessState,
    pub(super) memory: MemoryDescriptor,
    pub(super) threads: ListHead,
    real_parent: *mut ProcessControlBlock,
    parent: *mut ProcessControlBlock,
}

#[unsafe(no_mangle)]
pub(super) static mut INIT_TASK: ProcessControlBlock = ProcessControlBlock {
    identifier: 0,
    state: ProcessState::Running,
    memory: MemoryDescriptor::new(0),
    threads: ListHead::null(),
    real_parent: core::ptr::null_mut(),
    parent: core::ptr::null_mut(),
};
