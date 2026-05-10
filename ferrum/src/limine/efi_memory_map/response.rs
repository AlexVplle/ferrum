#[repr(C)]
pub struct LimineEfiMemoryMapResponse {
    pub memmap: *const u8,
    pub memmap_size: u64,
    pub desc_size: u64,
    pub desc_version: u64,
}
