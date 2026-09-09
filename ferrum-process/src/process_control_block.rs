use crate::state::ProcessState;
use crate::thread_control_block::ThreadControlBlock;
use core::ptr::NonNull;
use ferrum_core::linked_list::list::List;
use ferrum_mm::memory_descriptor::MemoryDescriptor;

pub struct ProcessControlBlock {
    identifier: u64,
    state: ProcessState,
    pub memory: MemoryDescriptor,
    pub threads: List<ThreadControlBlock>,
    real_parent: Option<NonNull<ProcessControlBlock>>,
    parent: Option<NonNull<ProcessControlBlock>>,
}

#[unsafe(no_mangle)]
pub static mut INIT_TASK: ProcessControlBlock = ProcessControlBlock {
    identifier: 0,
    state: ProcessState::Running,
    memory: MemoryDescriptor::new(0),
    threads: List::new(),
    real_parent: None,
    parent: None,
};
