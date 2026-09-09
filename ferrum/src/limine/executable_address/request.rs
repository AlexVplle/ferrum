use crate::limine::request::LimineRequest;
use super::response::LimineExecutableAddressResponse;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static EXECUTABLE_ADDRESS_REQUEST: LimineRequest<LimineExecutableAddressResponse> =
    LimineRequest::new([0x71ba76863cc55f63, 0xb2644a48c516a487], 0, ());
