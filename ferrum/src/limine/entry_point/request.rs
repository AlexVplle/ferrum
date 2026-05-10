use crate::limine::request::LimineRequest;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static ENTRY_POINT_REQUEST: LimineRequest<(), Option<unsafe extern "C" fn()>> =
    LimineRequest::new([0x13d86c7092a940e0, 0x2a368510d3fc3da6], 0, None);
