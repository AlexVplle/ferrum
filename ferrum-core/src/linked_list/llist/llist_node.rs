use super::super::link::Link;

pub struct LlistNode {
    pub(super) next: Link<LlistNode>,
}

unsafe impl Send for LlistNode {}

impl LlistNode {
    pub const fn new() -> Self {
        Self { next: None }
    }
}
