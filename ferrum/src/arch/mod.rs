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
pub use riscv64::{
    boot::{fdt_address, memory_regions, reserved_regions},
    console_write,
    constants::{MAX_PHYSMEM_BITS, PAGE_MASK, PAGE_OFFSET, PAGE_SHIFT, PAGE_SIZE, PHYSICAL_TO_VIRTUAL_OFFSET, SECTION_SIZE_BITS},
    context::Context,
    fixmap::fdt_virtual_address,
    paging::setup_direct_map,
    timer::{RiscvTimer as PlatformTimer, RISCV_TIMER as PLATFORM_TIMER},
    tlb::{flush_tlb_all, flush_tlb_kernel_range, flush_tlb_page},
};

#[cfg(target_arch = "x86_64")]
pub use x86_64::constants::{MAX_PHYSMEM_BITS, SECTION_SIZE_BITS};
