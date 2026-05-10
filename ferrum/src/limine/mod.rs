pub mod bootloader_info;
pub mod bootloader_performance;
pub mod memory_map;
pub mod module;
pub mod smbios;
pub mod date_at_boot;
pub mod efi_memory_map;
pub mod efi_system_table;
pub mod entry_point;
pub mod executable_cmdline;
pub mod executable_address;
pub mod executable_file;
pub mod file;
pub mod uuid;
pub mod firmware_type;
pub mod flanterm_fb;
pub mod framebuffer;
pub mod higher_half_direct_map;
pub mod keep_iommu;
pub mod multiprocessor;
pub mod paging_mode;
pub mod request;
pub mod response;
pub mod rsdp;
pub mod stack_size;
pub mod tsc_frequency;

#[used]
#[unsafe(link_section = ".limine_requests_start")]
pub static REQUESTS_START_MARKER: [u64; 4] = [
    0xf6b8f4b39de7d1ae,
    0xfab91a6940fcb9cf,
    0x785c6ed015d3e316,
    0x181e920a7852b9d9,
];

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static BASE_REVISION: [u64; 3] = [
    0xf9562b2d5c95a6c8,
    0x6a7b384944536bdc,
    6,
];

#[used]
#[unsafe(link_section = ".limine_requests_end")]
pub static REQUESTS_END_MARKER: [u64; 2] = [
    0xadc0e0531bb10d03,
    0x9572709f31764c62,
];
