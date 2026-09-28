use crate::arch::riscv::csr::{Scause, Sepc, Sstatus, Stval};

pub const TRAP_FRAME_SIZE: usize = core::mem::size_of::<TrapFrame>();
pub const SEPC_FRAME_SLOT: usize = core::mem::offset_of!(TrapFrame, sepc) / core::mem::size_of::<usize>();
pub const SCAUSE_FRAME_SLOT: usize = core::mem::offset_of!(TrapFrame, scause) / core::mem::size_of::<usize>();
pub const STVAL_FRAME_SLOT: usize = core::mem::offset_of!(TrapFrame, stval) / core::mem::size_of::<usize>();
pub const SSTATUS_FRAME_SLOT: usize = core::mem::offset_of!(TrapFrame, sstatus) / core::mem::size_of::<usize>();
pub const FLOAT_REGS_FRAME_SLOT: usize = core::mem::offset_of!(TrapFrame, float_registers) / core::mem::size_of::<usize>();
pub const FCSR_FRAME_SLOT: usize = core::mem::offset_of!(TrapFrame, float_csr) / core::mem::size_of::<usize>();

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
