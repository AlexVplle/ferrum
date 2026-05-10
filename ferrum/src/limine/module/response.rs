use crate::limine::file::LimineFile;

#[repr(C)]
pub struct LimineModuleResponse {
    pub module_count: u64,
    pub modules: *const *const LimineFile,
}
