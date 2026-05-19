use super::state::ProcessState;
use crate::arch::Context;

struct ProcessControlBlock {
    identifier: u64,
    state: ProcessState,
    context: Context,
    page_table_address: u64,
}
