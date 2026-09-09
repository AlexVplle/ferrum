#[cfg(target_arch = "riscv64")]
mod riscv64;

#[cfg(target_arch = "riscv64")]
pub use riscv64::{console_write, current_processor_id, current_task_id};

#[cfg(not(target_arch = "riscv64"))]
pub fn current_processor_id() -> usize { 0 }

#[cfg(not(target_arch = "riscv64"))]
pub fn current_task_id() -> u64 { 0 }
