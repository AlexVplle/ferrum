use core::ptr::NonNull;

pub unsafe trait Linked<L> {
    type Handle;

    fn into_ptr(handle: Self::Handle) -> NonNull<Self>;
    fn from_ptr(ptr: NonNull<Self>) -> Self::Handle;
    fn links(ptr: NonNull<Self>) -> NonNull<L>;
}
