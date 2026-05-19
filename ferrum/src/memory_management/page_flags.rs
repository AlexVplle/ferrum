use ferrum_macros::flag;

pub struct PageFlags(usize);

impl PageFlags {
    pub const fn new() -> Self {
        Self(0)
    }

    flag!(locked, 0);
    flag!(dirty, 1);
    flag!(referenced, 2);
    flag!(uptodate, 3);
    flag!(reserved, 4);
}
