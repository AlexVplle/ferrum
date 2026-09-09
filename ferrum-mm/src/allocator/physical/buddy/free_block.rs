use core::ptr::NonNull;

use ferrum_core::linked_list::linked::Linked;
use ferrum_core::linked_list::links::Links;

pub struct FreeBlock {
    pub links: Links<FreeBlock>,
}

unsafe impl Linked<Links<FreeBlock>> for FreeBlock {
    type Handle = NonNull<FreeBlock>;

    fn into_ptr(handle: NonNull<FreeBlock>) -> NonNull<FreeBlock> { handle }
    fn from_ptr(ptr: NonNull<FreeBlock>) -> NonNull<FreeBlock> { ptr }
    fn links(ptr: NonNull<FreeBlock>) -> NonNull<Links<FreeBlock>> {
        unsafe { NonNull::new_unchecked(&raw mut (*ptr.as_ptr()).links) }
    }
}
