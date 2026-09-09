use ferrum_mm::PhysicalAddress;

pub struct PlatformInfo {
    pub hart_count: usize,
    pub platform_level_interrupt_controller_address: Option<PhysicalAddress>,
    pub clock_frequency: Option<u64>,
}
