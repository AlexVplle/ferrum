#[repr(u64)]
pub enum LimineFirmwareType {
    X86Bios = 0,
    Efi32 = 1,
    Efi64 = 2,
    Sbi = 3,
}

#[repr(C)]
pub struct LimineFirmwareTypeResponse {
    pub firmware_type: LimineFirmwareType,
}
