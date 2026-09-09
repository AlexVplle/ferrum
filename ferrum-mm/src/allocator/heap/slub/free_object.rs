use core::ptr::NonNull;

use ferrum_core::linked_list::list::linked::Linked;
use ferrum_core::linked_list::list::links::Links;

pub struct FreeObject {
    pub links: Links<FreeObject>,
}

unsafe impl Linked<Links<FreeObject>> for FreeObject {
    fn links(ptr: NonNull<FreeObject>) -> NonNull<Links<FreeObject>> {
        unsafe { NonNull::new_unchecked(&raw mut (*ptr.as_ptr()).links) }
    }
}
