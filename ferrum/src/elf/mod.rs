mod constants;
mod rela;

pub use rela::Rela64;
use constants::R_INFO_TYPE_MASK;

#[cfg(target_arch = "riscv64")]
use crate::arch::riscv64::constants::R_RISCV_RELATIVE as R_RELATIVE;
#[cfg(target_arch = "riscv64")]
use crate::arch::riscv64::relocate::rela_dyn_range;

#[inline(never)]
pub fn apply_relocations() {
    let (rela_start, rela_end): (usize, usize) = rela_dyn_range();
    let mut ptr: *const Rela64 = rela_start as *const Rela64;
    let end: *const Rela64 = rela_end as *const Rela64;
    while ptr < end {
        let rela: &Rela64 = unsafe { &*ptr };
        let r_type: u32 = (rela.r_info & R_INFO_TYPE_MASK) as u32;
        if r_type == R_RELATIVE {
            unsafe { *(rela.r_offset as *mut i64) = rela.r_addend };
        }
        ptr = unsafe { ptr.add(1) };
    }
}
