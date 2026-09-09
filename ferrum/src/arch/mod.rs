pub trait Timer {
    fn init(&self);
    fn clock_frequency(&self) -> u64;
    fn current_time(&self) -> u64;
    fn schedule(&self, deadline: u64);
}

pub use ferrum_mm::arch::*;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "riscv64")]
pub mod riscv64;

#[cfg(target_arch = "riscv64")]
pub use riscv64::{
    console_write,
    context::Context,
    timer::{RiscvTimer as PlatformTimer, RISCV_TIMER as PLATFORM_TIMER},
};

#[cfg(target_arch = "riscv64")]
pub use ferrum_mm::arch::fixmap::{fdt_physical_address, fdt_virtual_address};
