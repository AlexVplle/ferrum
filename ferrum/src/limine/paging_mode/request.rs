use crate::limine::request::LimineRequest;
use super::paging_mode::LiminePagingMode;
use super::response::LiminePagingModeResponse;

#[repr(C)]
pub struct LiminePagingModeRequestData {
    pub mode: LiminePagingMode,
    pub max_mode: LiminePagingMode,
    pub min_mode: LiminePagingMode,
}

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static PAGING_MODE_REQUEST: LimineRequest<LiminePagingModeResponse, LiminePagingModeRequestData> =
    LimineRequest::new(
        [0x95c1a0edab0944cb, 0xa4e5cb3842f7488a],
        0,
        LiminePagingModeRequestData {
            mode: LiminePagingMode::Sv39,
            max_mode: LiminePagingMode::Sv39,
            min_mode: LiminePagingMode::Sv39,
        },
    );
