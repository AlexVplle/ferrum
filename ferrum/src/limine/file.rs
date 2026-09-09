use crate::limine::uuid::LimineUuid;

#[repr(u32)]
pub enum LimineMediaType {
    Generic = 0,
    Optical = 1,
    Tftp = 2,
}

#[repr(C)]
pub struct LimineFile {
    pub revision: u64,
    pub address: *const u8,
    pub size: u64,
    pub path: *const u8,
    pub cmdline: *const u8,
    pub media_type: LimineMediaType,
    pub unused: u32,
    pub tftp_ip: u32,
    pub tftp_port: u32,
    pub partition_index: u32,
    pub mbr_disk_id: u32,
    pub gpt_disk_uuid: LimineUuid,
    pub gpt_part_uuid: LimineUuid,
    pub part_uuid: LimineUuid,
}
