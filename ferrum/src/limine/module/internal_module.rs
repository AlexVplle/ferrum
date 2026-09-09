pub const LIMINE_INTERNAL_MODULE_REQUIRED: u64 = 1 << 0;
pub const LIMINE_INTERNAL_MODULE_COMPRESSED: u64 = 1 << 1;

#[repr(C)]
pub struct LimineInternalModule {
    pub path: *const u8,
    pub cmdline: *const u8,
    pub flags: u64,
}
