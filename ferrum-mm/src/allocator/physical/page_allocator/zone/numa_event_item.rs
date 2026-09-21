#[derive(Clone, Copy)]
#[repr(usize)]
pub enum NumaEventItem {
    Hit,
    Miss,
    Foreign,
    InterleaveHit,
    Local,
    Other,
}

pub const NR_NUMA_EVENT_ITEMS: usize = 6;
