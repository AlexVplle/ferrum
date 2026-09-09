use super::mp_info::LimineMpInfo;

#[repr(C)]
pub struct LimineMpResponse {
    pub flags: u64,
    pub bsp_hartid: u64,
    pub cpu_count: u64,
    pub cpus: *const *const LimineMpInfo,
}
