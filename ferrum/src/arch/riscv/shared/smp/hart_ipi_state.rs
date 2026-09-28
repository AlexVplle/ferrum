use core::ptr::NonNull;
use core::sync::atomic::{AtomicUsize, Ordering};

use ferrum_core::linked_list::llist::{Llist, LlistNode};

use super::call_single_data::CallSingleData;
use super::ipi_message::InterProcessorInterruptMessage;
use super::irq_work::InterruptRequestWork;

pub struct HartInterProcessorInterruptState {
    pending: AtomicUsize,
    pub call_single_queue: Llist,
    pub interrupt_request_work_queue: Llist,
}

impl HartInterProcessorInterruptState {
    pub const fn new() -> Self {
        Self {
            pending: AtomicUsize::new(0),
            call_single_queue: Llist::new(),
            interrupt_request_work_queue: Llist::new(),
        }
    }

    pub fn set_pending(&self, msg: InterProcessorInterruptMessage) {
        self.pending.fetch_or(msg.bit(), Ordering::Release);
    }

    pub fn take_pending(&self) -> usize {
        self.pending.swap(0, Ordering::Acquire)
    }

    pub fn enqueue_call(&self, data: NonNull<CallSingleData>) {
        let node: NonNull<LlistNode> = data.cast();
        unsafe { self.call_single_queue.push(node) };
    }

    pub fn enqueue_interrupt_request_work(&self, work: NonNull<InterruptRequestWork>) {
        let node: NonNull<LlistNode> = work.cast();
        unsafe { self.interrupt_request_work_queue.push(node) };
    }
}
