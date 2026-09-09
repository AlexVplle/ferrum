use core::ptr::NonNull;

use crate::arch::riscv64::constants::MAX_HARTS;

use super::call_single_data::CallSingleData;
use super::hart_ipi_state::HartInterProcessorInterruptState;
use super::ipi_message::InterProcessorInterruptMessage;
use super::irq_work::IrqWork;

pub struct InterProcessorInterruptController {
    states: [HartInterProcessorInterruptState; MAX_HARTS],
}

unsafe impl Send for InterProcessorInterruptController {}
unsafe impl Sync for InterProcessorInterruptController {}

impl InterProcessorInterruptController {
    pub const fn new() -> Self {
        Self {
            states: [const { HartInterProcessorInterruptState::new() }; MAX_HARTS],
        }
    }

    fn send_sbi_inter_processor_interrupt(hart_id: usize) {
        let mask: sbi::hart_mask::HartMask = sbi::hart_mask::HartMask::new(0).with_hart(hart_id);
        sbi::ipi::send_ipi(&mask);
    }

    pub fn send_reschedule(&self, hart_id: usize) {
        self.states[hart_id].set_pending(InterProcessorInterruptMessage::Reschedule);
        Self::send_sbi_inter_processor_interrupt(hart_id);
    }

    pub fn send_call_func(&self, hart_id: usize, data: NonNull<CallSingleData>) {
        self.states[hart_id].enqueue_call(data);
        self.states[hart_id].set_pending(InterProcessorInterruptMessage::CallFunc);
        Self::send_sbi_inter_processor_interrupt(hart_id);
    }

    pub fn send_cpu_stop(&self, hart_id: usize) {
        self.states[hart_id].set_pending(InterProcessorInterruptMessage::CpuStop);
        Self::send_sbi_inter_processor_interrupt(hart_id);
    }

    pub fn send_cpu_crash_stop(&self, hart_id: usize) {
        self.states[hart_id].set_pending(InterProcessorInterruptMessage::CpuCrashStop);
        Self::send_sbi_inter_processor_interrupt(hart_id);
    }

    pub fn send_irq_work(&self, hart_id: usize, work: NonNull<IrqWork>) {
        self.states[hart_id].enqueue_irq_work(work);
        self.states[hart_id].set_pending(InterProcessorInterruptMessage::IrqWork);
        Self::send_sbi_inter_processor_interrupt(hart_id);
    }

    pub fn send_timer(&self, hart_id: usize) {
        self.states[hart_id].set_pending(InterProcessorInterruptMessage::Timer);
        Self::send_sbi_inter_processor_interrupt(hart_id);
    }

    pub fn send_cpu_backtrace(&self, hart_id: usize) {
        self.states[hart_id].set_pending(InterProcessorInterruptMessage::CpuBacktrace);
        Self::send_sbi_inter_processor_interrupt(hart_id);
    }

    pub fn send_kgdb_roundup(&self, hart_id: usize) {
        self.states[hart_id].set_pending(InterProcessorInterruptMessage::KgdbRoundup);
        Self::send_sbi_inter_processor_interrupt(hart_id);
    }

    pub fn handle_inter_processor_interrupt(&self, hart_id: usize) {
        let state: &HartInterProcessorInterruptState = &self.states[hart_id];
        let mut remaining: usize = state.take_pending();

        while remaining != 0 {
            let bit: usize = remaining.trailing_zeros() as usize;
            remaining &= remaining - 1;

            match bit {
                x if x == InterProcessorInterruptMessage::Reschedule as usize => {
                    todo!()
                }
                x if x == InterProcessorInterruptMessage::CallFunc as usize => {
                    for node in state.call_single_queue.take() {
                        let data: NonNull<CallSingleData> = node.cast();
                        let call: &CallSingleData = unsafe { data.as_ref() };
                        (call.func)(call.data);
                    }
                }
                x if x == InterProcessorInterruptMessage::CpuStop as usize => {
                    sbi::hsm::hart_stop();
                }
                x if x == InterProcessorInterruptMessage::CpuCrashStop as usize => {
                    crate::arch::halt();
                }
                x if x == InterProcessorInterruptMessage::IrqWork as usize => {
                    for node in state.irq_work_queue.take() {
                        let work: NonNull<IrqWork> = node.cast();
                        let irq_work: &IrqWork = unsafe { work.as_ref() };
                        (irq_work.func)();
                    }
                }
                x if x == InterProcessorInterruptMessage::Timer as usize => {
                    todo!()
                }
                x if x == InterProcessorInterruptMessage::CpuBacktrace as usize => {
                    todo!()
                }
                x if x == InterProcessorInterruptMessage::KgdbRoundup as usize => {
                    todo!()
                }
                _ => {}
            }
        }
    }
}
