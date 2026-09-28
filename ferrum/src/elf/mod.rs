mod constants;
mod rela;

pub use rela::Rela;
use constants::R_INFO_TYPE_MASK;

#[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
use crate::arch::riscv::constants::R_RISCV_RELATIVE as R_RELATIVE;
#[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
use crate::arch::riscv::relocate::rela_dyn_range;

#[inline(never)]
pub fn apply_relocations() {
    let (rela_start, rela_end): (usize, usize) = rela_dyn_range();
    let mut ptr: *const Rela = rela_start as *const Rela;
    let end: *const Rela = rela_end as *const Rela;
    while ptr < end {
        let rela: &Rela = unsafe { &*ptr };
        let r_type: u32 = (rela.r_info & R_INFO_TYPE_MASK) as u32;
        if r_type == R_RELATIVE {
            unsafe { *(rela.r_offset as *mut isize) = rela.r_addend };
        }
        ptr = unsafe { ptr.add(1) };
    }
}
