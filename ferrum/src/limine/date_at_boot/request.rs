use crate::limine::request::LimineRequest;
use super::response::LimineDateAtBootResponse;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static DATE_AT_BOOT_REQUEST: LimineRequest<LimineDateAtBootResponse> =
    LimineRequest::new([0x502746e184c088aa, 0xfbc5ec83e6327893], 0, ());
