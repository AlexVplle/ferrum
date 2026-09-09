#[macro_use]
mod macros;

pub mod boot;
pub mod constants;
pub mod context;
pub mod csr;
pub mod early_paging;
pub mod fixmap;
pub mod paging;
pub mod page_table;
pub mod page_table_entry;
pub mod page_table_entry_flags;
pub mod plic;
pub mod timer;
pub mod tlb;
pub mod trap;

pub fn console_write(args: core::fmt::Arguments) {
    use core::fmt::Write;
    sbi::debug_console::DebugConsoleWriter.write_fmt(args).ok();
}

core::arch::global_asm!(
    ".section .text.boot,\"ax\"",
    ".global _start",
    "_start:",
    "fence.i",
    "csrw sie, zero",
    "csrw sip, zero",
    "mv s0, a0",
    "mv s1, a1",
    "bnez a0, .Lhalt",
    "la sp, INIT_KERNEL_STACK + {stack_size}",
    "la tp, INIT_THREAD",
    "call setup_virtual_memory",
    "li t0, {physical_to_virtual_offset}",
    "add sp, sp, t0",
    "add tp, tp, t0",
    "la t1, 1f",
    "add t1, t1, t0",
    "jr t1",
    "1:",
    "la t0, _trap_entry",
    "csrw stvec, t0",
    "mv a0, s0",
    "mv a1, s1",
    "call _riscv_entry",
    ".Lhalt:",
    "wfi",
    "j .Lhalt",
    stack_size = const crate::process::KERNEL_STACK_SIZE,
    physical_to_virtual_offset = const crate::arch::riscv64::constants::PHYSICAL_TO_VIRTUAL_OFFSET,
);

#[unsafe(no_mangle)]
extern "C" fn _riscv_entry(hartid: u64, fdt_address: u64) -> ! {
    if hartid != 0 {
        loop {
            unsafe { core::arch::asm!("wfi") };
        }
    }
    unsafe {
        fixmap::init();
        fixmap::map_fdt(fdt_address as usize);
    }
    boot::store_hart_id(hartid);
    boot::store_fdt_address(fdt_address);
    crate::process::init();
    crate::kernel_main();
}
