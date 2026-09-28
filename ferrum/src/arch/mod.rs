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
#[path = "riscv/rv64/mod.rs"]
pub mod riscv;
#[cfg(target_arch = "riscv32")]
#[path = "riscv/rv32/mod.rs"]
pub mod riscv;

#[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
pub use riscv::{
    context::Context,
    current_thread_pointer,
    halt,
    machine_power_off,
    machine_restart,
    plic,
    timer::{RiscvTimer as PlatformTimer, RISCV_TIMER as PLATFORM_TIMER},
    wait_seconds,
    SSTATUS_WRITE_MASK,
};

#[cfg(target_arch = "x86_64")]
pub use x86_64::{halt, machine_power_off, machine_restart, wait_seconds};

#[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
pub use ferrum_mm::arch::fixmap::{fdt_physical_address, fdt_virtual_address};
