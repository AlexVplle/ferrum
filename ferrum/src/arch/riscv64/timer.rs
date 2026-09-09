use core::sync::atomic::{AtomicU64, Ordering};

use super::boot;
use super::csr::{Sie, Sstatus, Time};
use crate::arch::Timer;

pub static RISCV_TIMER: RiscvTimer = RiscvTimer {
    clock_frequency: AtomicU64::new(0),
};

pub struct RiscvTimer {
    clock_frequency: AtomicU64,
}

impl Timer for RiscvTimer {
    fn init(&self) {
        let frequency: u64 =
            boot::clock_frequency().expect("timebase-frequency not found in FDT");
        self.clock_frequency.store(frequency, Ordering::Release);

        let mut sie: Sie = Sie::read();
        sie.set_supervisor_timer_interrupt_enable();
        Sie::write(sie);

        let mut sstatus: Sstatus = Sstatus::read();
        sstatus.set_supervisor_interrupt_enable();
        Sstatus::write(sstatus);
    }

    fn clock_frequency(&self) -> u64 {
        self.clock_frequency.load(Ordering::Acquire)
    }

    fn current_time(&self) -> u64 {
        Time::read().bits() as u64
    }

    fn schedule(&self, deadline: u64) {
        sbi::timer::set_timer(deadline);
    }
}
