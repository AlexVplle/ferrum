use crate::limine::request::LimineRequest;
use super::response::LimineSmbiosResponse;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static SMBIOS_REQUEST: LimineRequest<LimineSmbiosResponse> =
    LimineRequest::new([0xf40e8986dcf41ecf, 0x4abe831136e3ad6a], 0, ());
