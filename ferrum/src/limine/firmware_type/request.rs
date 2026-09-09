use crate::limine::request::LimineRequest;
use super::response::LimineFirmwareTypeResponse;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static FIRMWARE_TYPE_REQUEST: LimineRequest<LimineFirmwareTypeResponse> =
    LimineRequest::new([0x8c2f75d90bef28a8, 0x7045a4688eac00c3], 0, ());
