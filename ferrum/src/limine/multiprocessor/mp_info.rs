#[repr(C)]
pub struct LimineMpInfo {
    pub hartid: u64,
    pub reserved: u64,
    pub goto_address: Option<unsafe extern "C" fn(*const LimineMpInfo)>,
    pub extra_argument: u64,
}
