#![no_std]
#![no_main]
#![feature(ptr_alignment_type)]

extern crate alloc;

#[global_allocator]
static ALLOCATOR: ferrum_mm::allocator::heap::kmalloc::KmallocAllocator =
    ferrum_mm::allocator::heap::kmalloc::KmallocAllocator::new();

mod arch;
mod die;
mod elf;
mod panic;
mod power;
mod smp;
mod splash;
mod system_state;
mod timer;

#[cfg(target_arch = "x86_64")]
mod limine;

use ferrum_mm::{PhysicalAddress, VirtualAddress};

unsafe extern "C" {
    static _kernel_start: u8;
    static _kernel_end: u8;
}

pub fn kernel_main() -> ! {
    splash::print();
    ferrum_core::printkln!("[paging] mode={:?}", ferrum_mm::arch::paging_mode());
    ferrum_core::printkln!(
        "[smp] cpu_online_mask={:#b}",
        crate::smp::CPU_STATES.get_mask(crate::smp::CpuState::Online as usize)
    );
    lock_dependency::set_logger(|args| ferrum_core::printk!("{}", args));
    memory_init();
    timer::init();

    loop {}
}

fn memory_init() {
    let kernel_start: VirtualAddress = VirtualAddress::new(&raw const (_kernel_start) as usize);
    let kernel_end: VirtualAddress = VirtualAddress::new(&raw const (_kernel_end) as usize);
    let kernel_size: usize = kernel_end - kernel_start;
    let kernel_physical_start: PhysicalAddress = kernel_start.to_kernel_physical();
    ferrum_core::printkln!(
        "[kernel] start={} end={} size={} KiB",
        kernel_start,
        kernel_end,
        kernel_size / 1024
    );
    ferrum_mm::init::memory_manager_initialization(kernel_physical_start, kernel_size);
}
