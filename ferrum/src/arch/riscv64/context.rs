#[repr(C)]
pub struct Context {
    pub stack_pointer: usize,
    pub return_address: usize,
    pub saved_registers: [usize; 12],
    pub saved_fp_registers: [u64; 12],
    pub float_csr: u32,
}
