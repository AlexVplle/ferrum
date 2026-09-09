use core::ptr::NonNull;

use crate::linked_list::link::Link;
use crate::linked_list::list::List;
use crate::spinlock::{Spinlock, SpinlockGuard};

use super::block::NotifierBlock;
use super::result::NotifierResult;

pub struct AtomicNotifierChain {
    chain: Spinlock<List<NotifierBlock>>,
}

unsafe impl Send for AtomicNotifierChain {}
unsafe impl Sync for AtomicNotifierChain {}

impl AtomicNotifierChain {
    pub const fn new() -> Self {
        Self {
            chain: Spinlock::new(List::new()),
        }
    }

    pub fn register(&self, ptr: NonNull<NotifierBlock>) {
        let mut guard: SpinlockGuard<'_, List<NotifierBlock>> = self.chain.lock();
        guard.insert_sorted(ptr, |new, existing| new.priority > existing.priority);
    }

    pub unsafe fn unregister(&self, ptr: NonNull<NotifierBlock>) {
        let mut guard: SpinlockGuard<'_, List<NotifierBlock>> = self.chain.lock();
        unsafe { guard.remove(ptr) };
    }

    pub fn call_chain(&self, event: usize, data: *const ()) -> NotifierResult {
        let guard: SpinlockGuard<'_, List<NotifierBlock>> = self.chain.lock();
        let mut cursor: Link<NotifierBlock> = guard.front();
        while let Some(block_ptr) = cursor {
            let block: &NotifierBlock = unsafe { block_ptr.as_ref() };
            let next: Link<NotifierBlock> = block.links.next();
            let result: NotifierResult = (block.notifier_call)(block, event, data);
            if result.should_stop() {
                return result;
            }
            cursor = next;
        }
        NotifierResult::Done
    }
}
