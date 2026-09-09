use crate::arch::riscv64::csr::{Scause, Sepc, Sstatus, Stval};

#[repr(C)]
pub struct TrapFrame {
    pub regs: [usize; 32],
    pub sepc: Sepc,
    pub scause: Scause,
    pub stval: Stval,
    pub sstatus: Sstatus,
    pub float_registers: [u64; 32],
    pub float_csr: u32,
}
