use crate::limine::request::LimineRequest;
use super::response::LimineFramebufferResponse;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static FRAMEBUFFER_REQUEST: LimineRequest<LimineFramebufferResponse> =
    LimineRequest::new([0x9d5827dcd881dd75, 0xa3148604f6fab11b], 1, ());
