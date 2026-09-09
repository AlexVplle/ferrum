pub trait Timer {
    fn init(&self);
    fn clock_frequency(&self) -> usize;
    fn current_time(&self) -> usize;
    fn schedule(&self, deadline: usize);
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
    halt,
    machine_power_off,
    machine_restart,
    plic,
    timer::{RiscvTimer as PlatformTimer, RISCV_TIMER as PLATFORM_TIMER},
    wait_seconds,
};

#[cfg(target_arch = "x86_64")]
pub use x86_64::{halt, machine_power_off, machine_restart, wait_seconds};

#[cfg(target_arch = "riscv64")]
pub use ferrum_mm::arch::fixmap::{fdt_physical_address, fdt_virtual_address};
