use core::ptr;

use ferrum_core::notifier::AtomicNotifierChain;

pub static POWER_OFF_HANDLER_LIST: AtomicNotifierChain = AtomicNotifierChain::new();

pub fn do_power_off() -> ! {
    POWER_OFF_HANDLER_LIST.call_chain(0, ptr::null());
    crate::arch::machine_power_off();
}
