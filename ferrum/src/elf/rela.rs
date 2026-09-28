#[repr(C)]
pub struct Rela {
    pub r_offset: usize,
    pub r_info: usize,
    pub r_addend: isize,
}
