#[macro_use]
mod macros;

pub mod boot;
pub mod constants;
pub mod context;
pub mod csr;
pub mod plic;
pub mod relocate;
pub mod timer;
pub mod trap;

pub fn console_write(args: core::fmt::Arguments) {
    use core::fmt::Write;
    sbi::debug_console::DebugConsoleWriter.write_fmt(args).ok();
}

core::arch::global_asm!(
    ".section .text.boot,\"ax\"",
    ".global _start",
    "_start:",
    "bnez a0, .Lhalt",
    ".option push",
    ".option nopic",
    "la sp, INIT_KERNEL_STACK + {stack_size}",
    "la tp, INIT_THREAD",
    ".option pop",
    "tail physical_entry",
    ".Lhalt:",
    "wfi",
    "j .Lhalt",
    stack_size = const crate::process::KERNEL_STACK_SIZE,
);

#[unsafe(no_mangle)]
extern "C" fn physical_entry(hartid: usize, fdt: usize) -> ! {
    unsafe {
        core::arch::asm!(
            "fence.i",
            "csrw sie, zero",
            "csrw sip, zero",
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
            offset = const crate::arch::PHYSICAL_TO_VIRTUAL_OFFSET as i64,
            in("a0") hartid,
            in("a1") fdt,
            entry = sym virtual_entry,
            options(noreturn),
        );
    }
}

#[unsafe(no_mangle)]
extern "C" fn virtual_entry(hartid: u64, fdt_address: u64) -> ! {
    crate::elf::apply_relocations();
    unsafe {
        core::arch::asm!(
            "la {addr}, _trap_entry",
            "csrw stvec, {addr}",
            addr = out(reg) _,
            options(nostack),
        );
        ferrum_mm::arch::fixmap::init();
        ferrum_mm::arch::fixmap::map_fdt(fdt_address as usize);
    }
    boot::store_hart_id(hartid);
    boot::store_fdt_address(fdt_address);
    crate::process::init();
    crate::kernel_main();
}
