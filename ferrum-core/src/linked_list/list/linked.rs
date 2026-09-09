use core::ptr::NonNull;

pub unsafe trait Linked<L> {
    fn links(ptr: NonNull<Self>) -> NonNull<L>;
}
