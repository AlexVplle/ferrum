#[repr(C)]
pub struct Context {
    pub stack_pointer: usize,
    pub return_address: usize,
    pub saved_registers: [usize; 12],
    pub saved_fp_registers: [u64; 12],
    pub float_csr: u32,
}

pub fn current_thread_pointer() -> usize {
    let tp: usize;
    unsafe {
        core::arch::asm!("mv {}, tp", out(reg) tp, options(nostack, nomem));
    }
    tp
}
