use core::sync::atomic::{AtomicUsize, Ordering};

use super::boot;
use super::csr::{Sie, Sstatus, Time};
use crate::arch::Timer;

pub static RISCV_TIMER: RiscvTimer = RiscvTimer {
    clock_frequency: AtomicUsize::new(0),
};

pub struct RiscvTimer {
    clock_frequency: AtomicUsize,
}

impl Timer for RiscvTimer {
    fn init(&self) {
        let frequency: usize =
            boot::clock_frequency().expect("timebase-frequency not found in FDT") as usize;
        self.clock_frequency.store(frequency, Ordering::Release);

        let mut sie: Sie = Sie::read();
        sie.set_supervisor_timer_interrupt_enable();
        Sie::write(sie);

        let mut sstatus: Sstatus = Sstatus::read();
        sstatus.set_supervisor_interrupt_enable();
        Sstatus::write(sstatus);
    }

    fn clock_frequency(&self) -> usize {
        self.clock_frequency.load(Ordering::Acquire)
    }

    fn current_time(&self) -> usize {
        Time::read().bits()
    }

    fn schedule(&self, deadline: usize) {
        sbi::timer::set_timer(deadline as u64);
    }
}
