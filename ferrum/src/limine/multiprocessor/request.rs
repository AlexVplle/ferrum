use crate::limine::request::LimineRequest;
use super::response::LimineMpResponse;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static MP_REQUEST: LimineRequest<LimineMpResponse, u64> =
    LimineRequest::new([0x95a67b819a1b857e, 0xa0b61b723b6a73e0], 0, 0);
