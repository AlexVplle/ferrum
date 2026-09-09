pub mod mode;

use mode::RebootMode;

pub fn kernel_restart(mode: RebootMode) -> ! {
    crate::printkln!("[kernel] restarting...");
    crate::power::restart::do_restart(mode);
}

pub fn kernel_halt() -> ! {
    crate::printkln!("[kernel] halting...");
    crate::arch::halt();
}

pub fn kernel_power_off() -> ! {
    crate::printkln!("[kernel] powering off...");
    crate::power::shutdown::do_power_off();
}
