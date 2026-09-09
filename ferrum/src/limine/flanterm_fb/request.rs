use crate::limine::request::LimineRequest;
use super::response::LimineFlanTermFbResponse;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static FLANTERM_FB_REQUEST: LimineRequest<LimineFlanTermFbResponse> =
    LimineRequest::new([0x3259399fe7c5f126, 0xe01c1c8c5db9d1a9], 0, ());
