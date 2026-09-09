use super::memory_descriptor::MemoryDescriptor;
use super::state::ProcessState;
use super::thread_control_block::ThreadControlBlock;
use core::ptr::NonNull;
use ferrum_core::linked_list::list::List;

pub(super) struct ProcessControlBlock {
    identifier: u64,
    state: ProcessState,
    pub(super) memory: MemoryDescriptor,
    pub(super) threads: List<ThreadControlBlock>,
    real_parent: Option<NonNull<ProcessControlBlock>>,
    parent: Option<NonNull<ProcessControlBlock>>,
}

#[unsafe(no_mangle)]
pub(super) static mut INIT_TASK: ProcessControlBlock = ProcessControlBlock {
    identifier: 0,
    state: ProcessState::Running,
    memory: MemoryDescriptor::new(0),
    threads: List::new(),
    real_parent: None,
    parent: None,
};
