use core::ptr;

use ferrum_core::notifier::AtomicNotifierChain;

use crate::reboot::mode::RebootMode;

pub static RESTART_HANDLER_LIST: AtomicNotifierChain = AtomicNotifierChain::new();

pub fn do_restart(mode: RebootMode) -> ! {
    let cold: bool = match mode {
        RebootMode::Cold | RebootMode::Hard | RebootMode::Undefined => true,
        RebootMode::Warm | RebootMode::Soft | RebootMode::Gpio => false,
    };
    RESTART_HANDLER_LIST.call_chain(mode as usize, ptr::null());
    crate::arch::machine_restart(cold);
}
