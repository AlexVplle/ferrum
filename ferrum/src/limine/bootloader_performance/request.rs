use crate::limine::request::LimineRequest;
use super::response::LimineBootloaderPerformanceResponse;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static BOOTLOADER_PERFORMANCE_REQUEST: LimineRequest<LimineBootloaderPerformanceResponse> =
    LimineRequest::new([0x6b50ad9bf36d13ad, 0xdc4c7e88fc759e17], 0, ());
