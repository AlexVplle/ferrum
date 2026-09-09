use core::ptr::NonNull;

use ferrum_core::linked_list::list::linked::Linked;
use ferrum_core::linked_list::list::links::Links;

pub struct FreeBlock {
    pub links: Links<FreeBlock>,
}

unsafe impl Linked<Links<FreeBlock>> for FreeBlock {
    fn links(ptr: NonNull<FreeBlock>) -> NonNull<Links<FreeBlock>> {
        unsafe { NonNull::new_unchecked(&raw mut (*ptr.as_ptr()).links) }
    }
}
