use crate::limine::request::LimineRequest;
use super::internal_module::LimineInternalModule;
use super::response::LimineModuleResponse;

#[repr(C)]
pub struct LimineModuleRequestData {
    pub internal_module_count: u64,
    pub internal_modules: *const *const LimineInternalModule,
}

unsafe impl Sync for LimineModuleRequestData {}

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static MODULE_REQUEST: LimineRequest<LimineModuleResponse, LimineModuleRequestData> =
    LimineRequest::new(
        [0x3e7e279702be32af, 0xca1c4f3bd1280cee],
        0,
        LimineModuleRequestData {
            internal_module_count: 0,
            internal_modules: core::ptr::null(),
        },
    );
