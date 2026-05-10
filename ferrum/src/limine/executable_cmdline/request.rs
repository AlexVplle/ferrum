use crate::limine::request::LimineRequest;
use super::response::LimineExecutableCmdlineResponse;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static EXECUTABLE_CMDLINE_REQUEST: LimineRequest<LimineExecutableCmdlineResponse> =
    LimineRequest::new([0xad97e90e83f1101f, 0x2a59b1b8ac21315c], 0, ());
