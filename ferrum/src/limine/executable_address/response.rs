#[repr(C)]
pub struct LimineExecutableAddressResponse {
    pub physical_base: u64,
    pub virtual_base: u64,
}
