#[macro_use]
mod macros;

pub mod boot;
pub mod constants;
pub mod context;
pub mod csr;
pub mod plic;
pub mod timer;
pub mod trap;

use crate::memory_management::physical_address::PhysicalAddress;
use crate::memory_management::virtual_address::VirtualAddress;
use constants::HIGHER_HALF_DIRECT_MAP_BASE;

pub fn physical_to_virtual(address: PhysicalAddress) -> VirtualAddress {
    unsafe { VirtualAddress::new_unchecked(address.as_usize() + HIGHER_HALF_DIRECT_MAP_BASE) }
}

pub fn virtual_to_physical(address: VirtualAddress) -> PhysicalAddress {
    unsafe { PhysicalAddress::new_unchecked(address.as_usize() - HIGHER_HALF_DIRECT_MAP_BASE) }
}

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
    la t0, _trap_entry
    csrw stvec, t0
    call _riscv_entry
"#
);

#[unsafe(no_mangle)]
extern "C" fn _riscv_entry(hartid: u64, fdt_address: u64) -> ! {
    boot::store_hart_id(hartid);
    boot::store_fdt_address(fdt_address);
    crate::kernel_main();
}
