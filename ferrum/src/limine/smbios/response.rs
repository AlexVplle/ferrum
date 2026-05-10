#[repr(C)]
pub struct LimineSmbiosResponse {
    pub entry_32: *const u8,
    pub entry_64: *const u8,
}
