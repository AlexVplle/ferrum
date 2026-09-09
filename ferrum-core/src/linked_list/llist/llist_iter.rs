use core::ptr::NonNull;

use super::llist_node::LlistNode;

pub struct LlistIter {
    current: Option<NonNull<LlistNode>>,
}

impl LlistIter {
    pub fn new(head: Option<NonNull<LlistNode>>) -> Self {
        Self { current: head }
    }
}

impl core::iter::FusedIterator for LlistIter {}

impl Iterator for LlistIter {
    type Item = NonNull<LlistNode>;

    fn next(&mut self) -> Option<Self::Item> {
        let node: NonNull<LlistNode> = self.current?;
        self.current = unsafe { node.as_ref().next };
        Some(node)
    }
}
