use crate::limine::file::LimineFile;

#[repr(C)]
pub struct LimineExecutableFileResponse {
    pub executable_file: *const LimineFile,
}
