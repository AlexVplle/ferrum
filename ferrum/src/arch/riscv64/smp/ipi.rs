use core::ptr::NonNull;

use crate::arch::riscv64::constants::MAX_HARTS;

use super::call_single_data::CallSingleData;
use super::hart_ipi_state::HartIpiState;
use super::ipi_message::IpiMessage;
use super::irq_work::IrqWork;

static HART_IPI_STATES: [HartIpiState; MAX_HARTS] = [const { HartIpiState::new() }; MAX_HARTS];

fn send_sbi_ipi(hart_id: usize) {
    let mask: sbi::hart_mask::HartMask = sbi::hart_mask::HartMask::new(hart_id).with_hart(hart_id);
    sbi::ipi::send_ipi(&mask);
}

pub fn send_reschedule(hart_id: usize) {
    HART_IPI_STATES[hart_id].set_pending(IpiMessage::Reschedule);
    send_sbi_ipi(hart_id);
}

pub fn send_call_func(hart_id: usize, data: NonNull<CallSingleData>) {
    HART_IPI_STATES[hart_id].enqueue_call(data);
    HART_IPI_STATES[hart_id].set_pending(IpiMessage::CallFunc);
    send_sbi_ipi(hart_id);
}

pub fn send_cpu_stop(hart_id: usize) {
    HART_IPI_STATES[hart_id].set_pending(IpiMessage::CpuStop);
    send_sbi_ipi(hart_id);
}

pub fn send_cpu_crash_stop(hart_id: usize) {
    HART_IPI_STATES[hart_id].set_pending(IpiMessage::CpuCrashStop);
    send_sbi_ipi(hart_id);
}

pub fn send_irq_work(hart_id: usize, work: NonNull<IrqWork>) {
    HART_IPI_STATES[hart_id].enqueue_irq_work(work);
    HART_IPI_STATES[hart_id].set_pending(IpiMessage::IrqWork);
    send_sbi_ipi(hart_id);
}

pub fn send_timer(hart_id: usize) {
    HART_IPI_STATES[hart_id].set_pending(IpiMessage::Timer);
    send_sbi_ipi(hart_id);
}

pub fn send_cpu_backtrace(hart_id: usize) {
    HART_IPI_STATES[hart_id].set_pending(IpiMessage::CpuBacktrace);
    send_sbi_ipi(hart_id);
}

pub fn send_kgdb_roundup(hart_id: usize) {
    HART_IPI_STATES[hart_id].set_pending(IpiMessage::KgdbRoundup);
    send_sbi_ipi(hart_id);
}

pub fn handle_ipi(hart_id: usize) {
    let state: &HartIpiState = &HART_IPI_STATES[hart_id];
    let mut remaining: usize = state.take_pending();

    while remaining != 0 {
        let bit: usize = remaining.trailing_zeros() as usize;
        remaining &= remaining - 1;

        match bit {
            x if x == IpiMessage::Reschedule as usize => {
                todo!()
            }
            x if x == IpiMessage::CallFunc as usize => {
                for node in state.call_single_queue.take() {
                    let data: NonNull<CallSingleData> = node.cast();
                    let call: &CallSingleData = unsafe { data.as_ref() };
                    (call.func)(call.data);
                }
            }
            x if x == IpiMessage::CpuStop as usize => {
                sbi::hsm::hart_stop();
            }
            x if x == IpiMessage::CpuCrashStop as usize => {
                crate::arch::halt();
            }
            x if x == IpiMessage::IrqWork as usize => {
                for node in state.irq_work_queue.take() {
                    let work: NonNull<IrqWork> = node.cast();
                    let irq_work: &IrqWork = unsafe { work.as_ref() };
                    (irq_work.func)();
                }
            }
            x if x == IpiMessage::Timer as usize => {
                todo!()
            }
            x if x == IpiMessage::CpuBacktrace as usize => {
                todo!()
            }
            x if x == IpiMessage::KgdbRoundup as usize => {
                todo!()
            }
            _ => {}
        }
    }
}
