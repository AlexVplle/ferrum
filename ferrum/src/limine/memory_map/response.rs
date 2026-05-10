use super::entry::LimineMemoryMapEntry;

#[repr(C)]
pub struct LimineMemoryMapResponse {
    pub entry_count: u64,
    pub entries: *const *const LimineMemoryMapEntry,
}
