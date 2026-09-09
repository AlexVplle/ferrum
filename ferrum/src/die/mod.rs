mod events;

use core::ptr;

use ferrum_core::notifier::AtomicNotifierChain;

pub static DIE_NOTIFIER_HEAD: AtomicNotifierChain = AtomicNotifierChain::new();

pub fn die() -> ! {
    DIE_NOTIFIER_HEAD.call_chain(events::DIE, ptr::null());
    crate::arch::halt();
}
