mod events;

use core::panic::PanicInfo;
use core::ptr;
use core::sync::atomic::{AtomicIsize, Ordering};

use ferrum_core::notifier::AtomicNotifierChain;

pub static PANIC_NOTIFIER_HEAD: AtomicNotifierChain = AtomicNotifierChain::new();
pub static PANIC_TIMEOUT: AtomicIsize = AtomicIsize::new(0);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    match info.location() {
        Some(location) => crate::printkln!(
            "[panic] at {}:{}: {}",
            location.file(),
            location.line(),
            info.message()
        ),
        None => crate::printkln!("[panic] {}", info.message()),
    }

    PANIC_NOTIFIER_HEAD.call_chain(events::PANIC, ptr::null());

    let timeout: isize = PANIC_TIMEOUT.load(Ordering::Relaxed);

    if timeout < 0 {
        crate::arch::machine_restart(true);
    }

    if timeout > 0 {
        crate::arch::wait_seconds(timeout as usize);
        crate::arch::machine_restart(true);
    }

    crate::arch::halt();
}
