use ferrum_core::linked_list::list::List;
use crate::page::frame::Frame;
use super::super::constants::NR_PER_CPU_PAGES_LISTS;

pub struct PerCpuPagesInner {
    pub lists: [List<Frame>; NR_PER_CPU_PAGES_LISTS],
    pub count: usize,
}

impl PerCpuPagesInner {
    pub const fn new() -> Self {
        const EMPTY_LIST: List<Frame> = List::new();
        Self {
            lists: [EMPTY_LIST; NR_PER_CPU_PAGES_LISTS],
            count: 0,
        }
    }
}
