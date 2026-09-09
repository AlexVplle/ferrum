use crate::limine::request::LimineRequest;
use super::response::LimineExecutableFileResponse;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static EXECUTABLE_FILE_REQUEST: LimineRequest<LimineExecutableFileResponse> =
    LimineRequest::new([0xad97e90e83f1ed67, 0x31eb5d1c5ff23b69], 0, ());
