pub trait Timer {
    fn init(&self);
    fn clock_frequency(&self) -> u64;
    fn current_time(&self) -> u64;
    fn schedule(&self, deadline: u64);
}

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "riscv64")]
pub mod riscv64;

#[cfg(target_arch = "riscv64")]
pub use riscv64::boot::memory_regions;
#[cfg(target_arch = "riscv64")]
pub use riscv64::constants::{MAX_PHYSMEM_BITS, SECTION_SIZE_BITS};
#[cfg(target_arch = "riscv64")]
pub use riscv64::context::Context;
#[cfg(target_arch = "riscv64")]
pub use riscv64::timer::{RiscvTimer as PlatformTimer, RISCV_TIMER as PLATFORM_TIMER};
#[cfg(target_arch = "riscv64")]
pub use riscv64::virtual_to_physical;

#[cfg(target_arch = "x86_64")]
pub use x86_64::constants::{MAX_PHYSMEM_BITS, SECTION_SIZE_BITS};
