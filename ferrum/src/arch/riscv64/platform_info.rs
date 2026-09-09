use ferrum_core::singleton::Singleton;
use ferrum_mm::PhysicalAddress;

use super::boot;

pub struct PlatformInfo {
    hart_count: usize,
    platform_level_interrupt_controller_address: Option<PhysicalAddress>,
    clock_frequency: Option<u64>,
}

pub static PLATFORM_INFO: Singleton<PlatformInfo> = Singleton::new(PlatformInfo::new());

impl PlatformInfo {
    const fn new() -> Self {
        Self {
            hart_count: 1,
            platform_level_interrupt_controller_address: None,
            clock_frequency: None,
        }
    }

    pub fn init() {
        PLATFORM_INFO.init(Self {
            hart_count: boot::hart_count(),
            platform_level_interrupt_controller_address: boot::platform_level_interrupt_controller_address()
                .map(PhysicalAddress::new),
            clock_frequency: boot::clock_frequency(),
        });
    }

    pub fn hart_count(&self) -> usize {
        self.hart_count
    }

    pub fn platform_level_interrupt_controller_address(&self) -> PhysicalAddress {
        self.platform_level_interrupt_controller_address
            .expect("PLIC not found in FDT")
    }

    pub fn clock_frequency(&self) -> u64 {
        self.clock_frequency.expect("timebase-frequency not found in FDT")
    }
}
