use ferrum_macros::flag;

#[derive(Clone, Copy)]
pub(super) struct ProcessFlags(usize);

impl ProcessFlags {
    pub const fn new() -> Self {
        Self(0)
    }

    flag!(kernel_thread, 0);
    flag!(exiting, 1);
    flag!(signaled, 2);
    flag!(memalloc, 3);
    flag!(super_privileges, 4);
}
