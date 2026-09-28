#[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
mod riscv;

#[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
pub use riscv::{console_write, current_processor_id, current_task_id};

#[cfg(not(any(target_arch = "riscv64", target_arch = "riscv32")))]
pub fn current_processor_id() -> usize { 0 }

#[cfg(not(any(target_arch = "riscv64", target_arch = "riscv32")))]
pub fn current_task_id() -> u64 { 0 }
