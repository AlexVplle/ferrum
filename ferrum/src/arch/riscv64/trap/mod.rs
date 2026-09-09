pub mod exception;
pub mod frame;
pub mod interrupt;
pub mod trap;

use frame::TrapFrame;
use trap::Trap;

core::arch::global_asm!(
    r#"
    .section .text
    .attribute arch, "rv64gc"

    .macro ALLOCATE_TRAP_FRAME
    addi sp, sp, -({trap_frame_size})
    .endm

    .macro SWITCH_TO_KERNEL_STACK
    sd sp, {user_stack_pointer_offset}(tp)
    ld sp, {kernel_stack_pointer_offset}(tp)
    .endm

    .macro SAVE_USER_STACK_POINTER_AND_THREAD_POINTER
    ld t0,  {user_stack_pointer_offset}(tp)
    sd t0,  2*{register_size}(sp)
    csrr t0, sscratch
    sd t0,  4*{register_size}(sp)
    sd t0,  {user_thread_pointer_offset}(tp)
    csrw sscratch, zero
    .endm

    .macro SAVE_INT_REGS
    .irp n, 1, 3, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
    sd x\n, \n * {register_size}(sp)
    .endr
    .endm

    .macro RESTORE_INT_REGS
    .irp n, 1, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
    ld x\n, \n * {register_size}(sp)
    .endr
    .endm

    .macro SAVE_FLOAT_REGS
    .irp n, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
    fsd f\n, ({float_regs_frame_slot} + \n) * {register_size}(sp)
    .endr
    .endm

    .macro RESTORE_FLOAT_REGS
    .irp n, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
    fld f\n, ({float_regs_frame_slot} + \n) * {register_size}(sp)
    .endr
    .endm

    .macro SAVE_FLOAT_STATE
    SAVE_FLOAT_REGS
    csrr t0, fcsr
    sw t0, {fcsr_frame_slot}*{register_size}(sp)
    .endm

    .macro RESTORE_FLOAT_STATE
    lw t0, {fcsr_frame_slot}*{register_size}(sp)
    csrw fcsr, t0
    RESTORE_FLOAT_REGS
    .endm

    .global _trap_entry
    .align 4
_trap_entry:
    csrrw tp, sscratch, tp
    beqz tp, _from_kernel

_from_user:
    SWITCH_TO_KERNEL_STACK
    ALLOCATE_TRAP_FRAME

    sd x5,  5*{register_size}(sp)
    SAVE_USER_STACK_POINTER_AND_THREAD_POINTER
    SAVE_INT_REGS
    j _save_csrs

_from_kernel:
    csrr tp, sscratch
    csrw sscratch, zero
    ALLOCATE_TRAP_FRAME

    sd x5,  5*{register_size}(sp)
    addi t0, sp, {trap_frame_size}
    sd t0,  2*{register_size}(sp)
    sd x4,  4*{register_size}(sp)
    SAVE_INT_REGS

_save_csrs:
    csrr t0, sepc
    sd t0, {sepc_frame_slot}*{register_size}(sp)
    csrr t0, scause
    sd t0, {scause_frame_slot}*{register_size}(sp)
    csrr t0, stval
    sd t0, {stval_frame_slot}*{register_size}(sp)
    csrr t0, sstatus
    sd t0, {sstatus_frame_slot}*{register_size}(sp)

    SAVE_FLOAT_STATE

    mv a0, sp
    call trap_handler

    ld t0, {sstatus_frame_slot}*{register_size}(sp)
    andi t0, t0, {sstatus_spp_bit}
    beqz t0, _exit_user

_exit_kernel:
    ld t0, {sepc_frame_slot}*{register_size}(sp)
    csrw sepc, t0
    ld t0, {sstatus_frame_slot}*{register_size}(sp)
    csrw sstatus, t0
    RESTORE_FLOAT_STATE

    RESTORE_INT_REGS
    ld x4,  4*{register_size}(sp)
    ld x2,  2*{register_size}(sp)
    sret

_exit_user:
    ld t0, {sepc_frame_slot}*{register_size}(sp)
    csrw sepc, t0
    ld t0, {sstatus_frame_slot}*{register_size}(sp)
    csrw sstatus, t0
    RESTORE_FLOAT_STATE

    csrw sscratch, tp
    RESTORE_INT_REGS
    ld x4,  4*{register_size}(sp)
    ld x2,  2*{register_size}(sp)
    sret
"#,
    trap_frame_size = const frame::TRAP_FRAME_SIZE,
    kernel_stack_pointer_offset = const ferrum_process::KERNEL_STACK_POINTER_OFFSET,
    user_stack_pointer_offset = const ferrum_process::USER_STACK_POINTER_OFFSET,
    user_thread_pointer_offset = const ferrum_process::USER_THREAD_POINTER_OFFSET,
    register_size = const core::mem::size_of::<usize>(),
    sepc_frame_slot = const frame::SEPC_FRAME_SLOT,
    scause_frame_slot = const frame::SCAUSE_FRAME_SLOT,
    stval_frame_slot = const frame::STVAL_FRAME_SLOT,
    sstatus_frame_slot = const frame::SSTATUS_FRAME_SLOT,
    float_regs_frame_slot = const frame::FLOAT_REGS_FRAME_SLOT,
    fcsr_frame_slot = const frame::FCSR_FRAME_SLOT,
    sstatus_spp_bit = const crate::arch::riscv64::csr::sstatus::SUPERVISOR_PREVIOUS_PRIVILEGE_BIT,
);

#[unsafe(no_mangle)]
extern "C" fn trap_handler(frame: *mut TrapFrame) {
    let frame: &mut TrapFrame = unsafe { &mut *frame };
    match Trap::from(frame.scause) {
        Trap::Interrupt(interrupt) => interrupt.handle(),
        Trap::Exception(exception) => exception.handle(frame),
    }
}
