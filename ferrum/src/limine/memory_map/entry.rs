use super::entry_type::LimineMemoryMapEntryType;

#[repr(C)]
pub struct LimineMemoryMapEntry {
    pub base: u64,
    pub length: u64,
    pub entry_type: LimineMemoryMapEntryType,
}
