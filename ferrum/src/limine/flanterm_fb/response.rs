use super::init_params::LimineFlanTermFbInitParams;

#[repr(C)]
pub struct LimineFlanTermFbResponse {
    pub entry_count: u64,
    pub entries: *const *const LimineFlanTermFbInitParams,
}
