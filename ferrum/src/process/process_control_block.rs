use super::state::ProcessState;

struct ProcessControlBlock {
    identifier: u64,
    state: ProcessState,
}
