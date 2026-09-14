#[derive(Clone, Copy)]
#[repr(usize)]
pub enum LruList {
    InactiveAnon,
    ActiveAnon,
    InactiveFile,
    ActiveFile,
    Unevictable,
}

pub const NR_LRU_LISTS: usize = 5;
