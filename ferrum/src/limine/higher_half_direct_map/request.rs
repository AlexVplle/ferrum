use crate::limine::request::LimineRequest;
use super::response::LimineHhdmResponse;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static HHDM_REQUEST: LimineRequest<LimineHhdmResponse> =
    LimineRequest::new([0x48dcf1cb8ad2b852, 0x63984e959a98244b], 0, ());
