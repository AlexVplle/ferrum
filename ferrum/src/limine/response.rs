#[repr(C)]
pub struct LimineResponse<T> {
    pub revision: u64,
    pub data: T,
}
