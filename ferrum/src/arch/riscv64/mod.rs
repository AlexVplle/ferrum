core::arch::global_asm!(
    r#"
    .section .bss
    .align 12
_kernel_stack_bottom:
    .space 65536
_kernel_stack_top:

    .section .text
    .global _start
_start:
    la sp, _kernel_stack_top
    call _riscv_entry
"#
);


#[unsafe(no_mangle)]
extern "C" fn _riscv_entry(_hartid: u64, _fdt_address: u64) -> ! {
    crate::kernel_main();
}
