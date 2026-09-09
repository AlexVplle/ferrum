mod events;

use core::panic::PanicInfo;
use core::ptr;

use ferrum_core::notifier::AtomicNotifierChain;

pub static PANIC_NOTIFIER_HEAD: AtomicNotifierChain = AtomicNotifierChain::new();

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    PANIC_NOTIFIER_HEAD.call_chain(events::PANIC, ptr::null());
    crate::arch::halt();
}
