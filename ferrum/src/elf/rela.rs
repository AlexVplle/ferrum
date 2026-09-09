#[repr(C)]
pub struct Rela64 {
    pub r_offset: usize,
    pub r_info: usize,
    pub r_addend: i64,
}
