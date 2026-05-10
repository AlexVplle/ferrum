use crate::limine::request::LimineRequest;
use super::response::LimineBootloaderInfoResponse;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static BOOTLOADER_INFO_REQUEST: LimineRequest<LimineBootloaderInfoResponse> =
    LimineRequest::new([0xf55038d8e2a1202f, 0x279426fcf5f59740], 0, ());
