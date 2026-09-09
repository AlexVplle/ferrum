#[repr(C)]
pub struct LimineBootloaderPerformanceResponse {
    pub reset_usec: u64,
    pub init_usec: u64,
    pub exec_usec: u64,
}
