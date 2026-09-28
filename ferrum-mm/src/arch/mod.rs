#[cfg(target_arch = "riscv64")]
#[path = "riscv/rv64/mod.rs"]
mod riscv;
#[cfg(target_arch = "riscv32")]
#[path = "riscv/rv32/mod.rs"]
mod riscv;
#[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
pub use riscv::*;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
