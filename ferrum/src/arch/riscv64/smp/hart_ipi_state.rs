use core::ptr::NonNull;
use core::sync::atomic::{AtomicUsize, Ordering};

use ferrum_core::linked_list::llist::{Llist, LlistNode};

use super::call_single_data::CallSingleData;
use super::ipi_message::IpiMessage;
use super::irq_work::IrqWork;

pub struct HartIpiState {
    pending: AtomicUsize,
    pub call_single_queue: Llist,
    pub irq_work_queue: Llist,
}

impl HartIpiState {
    pub const fn new() -> Self {
        Self {
            pending: AtomicUsize::new(0),
            call_single_queue: Llist::new(),
            irq_work_queue: Llist::new(),
        }
    }

    pub fn set_pending(&self, msg: IpiMessage) {
        self.pending.fetch_or(msg.bit(), Ordering::Release);
    }

    pub fn take_pending(&self) -> usize {
        self.pending.swap(0, Ordering::Acquire)
    }

    pub fn enqueue_call(&self, data: NonNull<CallSingleData>) {
        let node: NonNull<LlistNode> = data.cast();
        unsafe { self.call_single_queue.push(node) };
    }

    pub fn enqueue_irq_work(&self, work: NonNull<IrqWork>) {
        let node: NonNull<LlistNode> = work.cast();
        unsafe { self.irq_work_queue.push(node) };
    }
}
