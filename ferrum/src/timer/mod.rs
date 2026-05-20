mod constants;

use constants::TICK_INTERVAL_S;
use crate::printkln;

use crate::arch::{PLATFORM_TIMER, Timer};

fn tick_interval() -> u64 {
    PLATFORM_TIMER.clock_frequency() * TICK_INTERVAL_S
}

pub fn init() {
    PLATFORM_TIMER.init();
    printkln!("[timer] clock frequency: {} Hz", PLATFORM_TIMER.clock_frequency());
    schedule_next_tick();
}

pub fn schedule_next_tick() {
    let deadline: u64 = PLATFORM_TIMER.current_time() + tick_interval();
    PLATFORM_TIMER.schedule(deadline);
}

pub fn on_tick() {
    printkln!("[timer] tick at t={}", PLATFORM_TIMER.current_time());
    schedule_next_tick();
}
