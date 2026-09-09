use core::ptr::NonNull;

use crate::linked_list::list::linked::Linked;
use crate::linked_list::list::links::Links;

use super::notifier_fn::NotifierFn;

pub struct NotifierBlock {
    pub links: Links<NotifierBlock>,
    pub notifier_call: NotifierFn,
    pub priority: i32,
}

impl NotifierBlock {
    pub const fn new(notifier_call: NotifierFn, priority: i32) -> Self {
        Self {
            links: Links::new(),
            notifier_call,
            priority,
        }
    }
}

unsafe impl Linked<Links<NotifierBlock>> for NotifierBlock {
    fn links(ptr: NonNull<NotifierBlock>) -> NonNull<Links<NotifierBlock>> {
        unsafe { NonNull::new_unchecked(&raw mut (*ptr.as_ptr()).links) }
    }
}
