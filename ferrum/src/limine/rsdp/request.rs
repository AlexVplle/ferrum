use crate::limine::request::LimineRequest;
use super::response::LimineRsdpResponse;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static RSDP_REQUEST: LimineRequest<LimineRsdpResponse> =
    LimineRequest::new([0xc5e77b6b397e7b43, 0x27637845accdcf3c], 0, ());
