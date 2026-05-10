use super::paging_mode::LiminePagingMode;

#[repr(C)]
pub struct LiminePagingModeResponse {
    pub mode: LiminePagingMode,
}
