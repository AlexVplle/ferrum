#[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
mod riscv;
#[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
pub use riscv::*;
