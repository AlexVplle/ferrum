pub mod exception;
pub mod frame;
pub mod interrupt;
pub mod trap;

use frame::TrapFrame;
use trap::Trap;

#[unsafe(no_mangle)]
extern "C" fn trap_handler(frame: *mut TrapFrame) {
    let frame: &mut TrapFrame = unsafe { &mut *frame };
    match Trap::from(frame.scause) {
        Trap::Interrupt(interrupt) => interrupt.handle(),
        Trap::Exception(exception) => exception.handle(frame),
    }
}
