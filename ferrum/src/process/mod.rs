mod constants;
mod flags;
mod kernel_stack;
pub mod memory_descriptor;
mod process_control_block;
mod process_list;
mod state;
mod thread_control_block;

pub(crate) use constants::KERNEL_STACK_SIZE;

pub(crate) fn init() {
    thread_control_block::init();
}
