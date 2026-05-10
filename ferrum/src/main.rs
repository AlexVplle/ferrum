#![no_std]
#![no_main]

mod arch;
mod process;

#[cfg(target_arch = "x86_64")]
mod limine;

use core::panic::PanicInfo;

pub fn kernel_main() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
