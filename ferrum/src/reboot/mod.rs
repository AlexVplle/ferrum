mod events;
pub mod mode;

use core::ptr;

use ferrum_core::notifier::AtomicNotifierChain;

use mode::RebootMode;

pub static REBOOT_NOTIFIER_HEAD: AtomicNotifierChain = AtomicNotifierChain::new();

pub fn kernel_restart(mode: RebootMode) -> ! {
    REBOOT_NOTIFIER_HEAD.call_chain(events::SYS_RESTART, ptr::null());
    let cold: bool = match mode {
        RebootMode::Cold | RebootMode::Hard | RebootMode::Undefined => true,
        RebootMode::Warm | RebootMode::Soft | RebootMode::Gpio => false,
    };
    crate::arch::machine_restart(cold);
}

pub fn kernel_halt() -> ! {
    REBOOT_NOTIFIER_HEAD.call_chain(events::SYS_HALT, ptr::null());
    crate::arch::halt();
}

pub fn kernel_power_off() -> ! {
    REBOOT_NOTIFIER_HEAD.call_chain(events::SYS_POWER_OFF, ptr::null());
    crate::arch::machine_power_off();
}
