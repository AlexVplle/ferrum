use crate::limine::request::LimineRequest;
use super::response::LimineTscFrequencyResponse;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static TSC_FREQUENCY_REQUEST: LimineRequest<LimineTscFrequencyResponse> =
    LimineRequest::new([0x10f2ee1d87d195e4, 0xf747a2b78f6ddb31], 0, ());
