use crate::limine::request::LimineRequest;
use super::response::LimineEfiSystemTableResponse;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static EFI_SYSTEM_TABLE_REQUEST: LimineRequest<LimineEfiSystemTableResponse> =
    LimineRequest::new([0x4b837d6d470c2cb1, 0x1082900d60d1dd72], 0, ());
