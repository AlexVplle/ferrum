pub mod cpumask;

use cpumask::AtomicCpuMask;

pub static CPU_ONLINE_MASK: AtomicCpuMask = AtomicCpuMask::new();
