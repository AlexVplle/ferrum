use crate::limine::request::LimineRequest;
use super::response::LimineEfiMemoryMapResponse;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static EFI_MEMORY_MAP_REQUEST: LimineRequest<LimineEfiMemoryMapResponse> =
    LimineRequest::new([0x7df62a431d6872d5, 0xa4fcdfb3e57306c8], 0, ());
