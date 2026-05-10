use crate::limine::request::LimineRequest;
use super::response::LimineStackSizeResponse;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static STACK_SIZE_REQUEST: LimineRequest<LimineStackSizeResponse, u64> =
    LimineRequest::new([0x224ef0460a8e8926, 0xe1cb0fc25f46ea3d], 0, 0);
