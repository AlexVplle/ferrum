#[repr(C)]
pub struct LimineBootloaderInfoResponse {
    pub name: *const u8,
    pub version: *const u8,
}
