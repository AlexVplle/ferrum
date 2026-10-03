#[macro_use]
mod macros;

pub mod boot;
pub mod constants;
pub mod context;
pub mod csr;
pub mod platform_info;
pub mod plic;
pub mod relocate;
pub mod smp;
pub mod timer;
pub mod trap;

pub fn current_thread_pointer() -> *mut u8 {
    let ptr: *mut u8;
    unsafe {
        core::arch::asm!("mv {}, tp", out(reg) ptr);
    }
    ptr
}

pub fn halt() -> ! {
    loop {
        unsafe { core::arch::asm!("wfi") };
    }
}

pub fn machine_restart(cold: bool) -> ! {
    let reset_type: u32 = if cold {
        sbi::srst::reset_type::COLD_REBOOT
    } else {
        sbi::srst::reset_type::WARM_REBOOT
    };
    sbi::srst::system_reset(reset_type, sbi::srst::reset_reason::NO_REASON);
}

pub fn machine_power_off() -> ! {
    sbi::srst::system_reset(sbi::srst::reset_type::SHUTDOWN, sbi::srst::reset_reason::NO_REASON);
}

pub fn wait_seconds(n: usize) {
    use crate::arch::{PLATFORM_TIMER, Timer};
    let frequency: usize = PLATFORM_TIMER.clock_frequency();
    if frequency == 0 {
        return;
    }
    let start: usize = PLATFORM_TIMER.current_time();
    let end: usize = start.wrapping_add(frequency * n);
    while PLATFORM_TIMER.current_time() < end {
        unsafe { core::arch::asm!("wfi") };
    }
}


core::arch::global_asm!(
    ".section .text.boot,\"ax\"",
    #[cfg(target_arch = "riscv64")]
    ".attribute arch, \"rv64gc\"",
    #[cfg(target_arch = "riscv32")]
    ".attribute arch, \"rv32imafc_zaamo_zalrsc\"",
    ".global _start",
    "_start:",
    ".option push",
    ".option nopic",
    "la t0, BOOT_HART_CLAIMED",
    "li t1, 1",
    "amoswap.w.aq t2, t1, (t0)",
    "bnez t2, .Lhalt",
    "la sp, INIT_KERNEL_STACK + {stack_size}",
    "la tp, INIT_THREAD",
    ".option pop",
    "tail physical_entry",
    ".Lhalt:",
    "wfi",
    "j .Lhalt",
    stack_size = const ferrum_process::KERNEL_STACK_SIZE,
);

#[unsafe(no_mangle)]
static BOOT_HART_CLAIMED: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

#[unsafe(no_mangle)]
extern "C" fn physical_entry(hartid: usize, fdt: usize) -> ! {
    unsafe {
        core::arch::asm!(
            "fence.i",
            "csrw sie, zero",
            "csrw sip, zero",
            "csrw sscratch, zero",
            options(nostack),
        );
    }
    ferrum_mm::arch::paging::early_paging::setup_virtual_memory();
    unsafe {
        core::arch::asm!(
            "li t1, {offset}",
            "add sp, sp, t1",
            "add tp, tp, t1",
            ".option push",
            ".option nopic",
            "la t0, 1f",
            ".option pop",
            "add t0, t0, t1",
            "jr t0",
            "1:",
            "tail {entry}",
            offset = const crate::arch::PHYSICAL_TO_VIRTUAL_OFFSET as isize,
            in("a0") hartid,
            in("a1") fdt,
            entry = sym virtual_entry,
            options(noreturn),
        );
    }
}

#[unsafe(no_mangle)]
extern "C" fn virtual_entry(hartid: usize, fdt_address: usize) -> ! {
    crate::elf::apply_relocations();
    unsafe {
        core::arch::asm!(
            "la {addr}, _trap_entry",
            "csrw stvec, {addr}",
            addr = out(reg) _,
            options(nostack),
        );
        ferrum_mm::arch::fixmap::init();
        ferrum_mm::arch::fixmap::map_fdt(ferrum_mm::PhysicalAddress::new(fdt_address));
    }
    ferrum_process::ThreadControlBlock::current().set_processor_id(hartid);
    csr::Sie::set_bits(csr::sie::SUPERVISOR_SOFTWARE_INTERRUPT_ENABLE);
    platform_info::PlatformInfo::init();
    crate::smp::CPU_STATES.cpu_set(hartid, crate::smp::CpuState::Online);
    ferrum_process::init();
    crate::kernel_main();
}
