use ferrum_macros::flag;

pub struct PageTableEntryFlags(usize);

impl PageTableEntryFlags {
    pub const fn new() -> Self {
        Self(0)
    }

    flag!(valid, 0);
    flag!(read, 1);
    flag!(write, 2);
    flag!(execute, 3);
    flag!(user, 4);
    flag!(global, 5);
    flag!(accessed, 6);
    flag!(dirty, 7);

    pub const fn bits(&self) -> usize {
        self.0
    }
}
