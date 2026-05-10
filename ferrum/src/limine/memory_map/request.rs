use crate::limine::request::LimineRequest;
use super::response::LimineMemoryMapResponse;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static MEMORY_MAP_REQUEST: LimineRequest<LimineMemoryMapResponse> =
    LimineRequest::new([0x67cf3d9d378a806f, 0xe304acdfc50c3c62], 0, ());
