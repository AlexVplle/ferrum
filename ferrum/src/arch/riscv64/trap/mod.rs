pub mod exception;
pub mod frame;
pub mod interrupt;
pub mod trap;

use frame::TrapFrame;
use trap::Trap;

core::arch::global_asm!(
    r#"
    .section .text
    .attribute arch, "rv64imafdc"
    .global _trap_entry
    .align 4
_trap_entry:
    addi sp, sp, -(69*8)

    sd x1,  1*8(sp)
    addi t0, sp, 69*8
    sd t0,  2*8(sp)
    sd x3,  3*8(sp)
    sd x4,  4*8(sp)
    sd x5,  5*8(sp)
    sd x6,  6*8(sp)
    sd x7,  7*8(sp)
    sd x8,  8*8(sp)
    sd x9,  9*8(sp)
    sd x10, 10*8(sp)
    sd x11, 11*8(sp)
    sd x12, 12*8(sp)
    sd x13, 13*8(sp)
    sd x14, 14*8(sp)
    sd x15, 15*8(sp)
    sd x16, 16*8(sp)
    sd x17, 17*8(sp)
    sd x18, 18*8(sp)
    sd x19, 19*8(sp)
    sd x20, 20*8(sp)
    sd x21, 21*8(sp)
    sd x22, 22*8(sp)
    sd x23, 23*8(sp)
    sd x24, 24*8(sp)
    sd x25, 25*8(sp)
    sd x26, 26*8(sp)
    sd x27, 27*8(sp)
    sd x28, 28*8(sp)
    sd x29, 29*8(sp)
    sd x30, 30*8(sp)
    sd x31, 31*8(sp)

    csrr t0, sepc
    sd t0, 32*8(sp)
    csrr t0, scause
    sd t0, 33*8(sp)
    csrr t0, stval
    sd t0, 34*8(sp)
    csrr t0, sstatus
    sd t0, 35*8(sp)

    fsd f0,  36*8(sp)
    fsd f1,  37*8(sp)
    fsd f2,  38*8(sp)
    fsd f3,  39*8(sp)
    fsd f4,  40*8(sp)
    fsd f5,  41*8(sp)
    fsd f6,  42*8(sp)
    fsd f7,  43*8(sp)
    fsd f8,  44*8(sp)
    fsd f9,  45*8(sp)
    fsd f10, 46*8(sp)
    fsd f11, 47*8(sp)
    fsd f12, 48*8(sp)
    fsd f13, 49*8(sp)
    fsd f14, 50*8(sp)
    fsd f15, 51*8(sp)
    fsd f16, 52*8(sp)
    fsd f17, 53*8(sp)
    fsd f18, 54*8(sp)
    fsd f19, 55*8(sp)
    fsd f20, 56*8(sp)
    fsd f21, 57*8(sp)
    fsd f22, 58*8(sp)
    fsd f23, 59*8(sp)
    fsd f24, 60*8(sp)
    fsd f25, 61*8(sp)
    fsd f26, 62*8(sp)
    fsd f27, 63*8(sp)
    fsd f28, 64*8(sp)
    fsd f29, 65*8(sp)
    fsd f30, 66*8(sp)
    fsd f31, 67*8(sp)
    csrr t0, fcsr
    sw t0, 68*8(sp)

    mv a0, sp
    call trap_handler

    ld t0, 32*8(sp)
    csrw sepc, t0

    lw t0, 68*8(sp)
    csrw fcsr, t0
    fld f0,  36*8(sp)
    fld f1,  37*8(sp)
    fld f2,  38*8(sp)
    fld f3,  39*8(sp)
    fld f4,  40*8(sp)
    fld f5,  41*8(sp)
    fld f6,  42*8(sp)
    fld f7,  43*8(sp)
    fld f8,  44*8(sp)
    fld f9,  45*8(sp)
    fld f10, 46*8(sp)
    fld f11, 47*8(sp)
    fld f12, 48*8(sp)
    fld f13, 49*8(sp)
    fld f14, 50*8(sp)
    fld f15, 51*8(sp)
    fld f16, 52*8(sp)
    fld f17, 53*8(sp)
    fld f18, 54*8(sp)
    fld f19, 55*8(sp)
    fld f20, 56*8(sp)
    fld f21, 57*8(sp)
    fld f22, 58*8(sp)
    fld f23, 59*8(sp)
    fld f24, 60*8(sp)
    fld f25, 61*8(sp)
    fld f26, 62*8(sp)
    fld f27, 63*8(sp)
    fld f28, 64*8(sp)
    fld f29, 65*8(sp)
    fld f30, 66*8(sp)
    fld f31, 67*8(sp)

    ld x1,  1*8(sp)
    ld x3,  3*8(sp)
    ld x4,  4*8(sp)
    ld x5,  5*8(sp)
    ld x6,  6*8(sp)
    ld x7,  7*8(sp)
    ld x8,  8*8(sp)
    ld x9,  9*8(sp)
    ld x10, 10*8(sp)
    ld x11, 11*8(sp)
    ld x12, 12*8(sp)
    ld x13, 13*8(sp)
    ld x14, 14*8(sp)
    ld x15, 15*8(sp)
    ld x16, 16*8(sp)
    ld x17, 17*8(sp)
    ld x18, 18*8(sp)
    ld x19, 19*8(sp)
    ld x20, 20*8(sp)
    ld x21, 21*8(sp)
    ld x22, 22*8(sp)
    ld x23, 23*8(sp)
    ld x24, 24*8(sp)
    ld x25, 25*8(sp)
    ld x26, 26*8(sp)
    ld x27, 27*8(sp)
    ld x28, 28*8(sp)
    ld x29, 29*8(sp)
    ld x30, 30*8(sp)
    ld x31, 31*8(sp)
    ld x2,  2*8(sp)

    sret
"#
);

#[unsafe(no_mangle)]
extern "C" fn trap_handler(frame: *mut TrapFrame) {
    let frame: &mut TrapFrame = unsafe { &mut *frame };
    match Trap::from(frame.scause) {
        Trap::Interrupt(interrupt) => interrupt.handle(),
        Trap::Exception(exception) => exception.handle(frame),
    }
}
