use core::cell::UnsafeCell;
use core::mem;

use super::link::Link;
use super::links_inner::LinksInner;

pub struct Links<T> {
    inner: UnsafeCell<LinksInner<T>>,
}

unsafe impl<T> Send for Links<T> {}
unsafe impl<T> Sync for Links<T> {}

impl<T> Links<T> {
    pub const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(LinksInner::new()),
        }
    }

    pub fn next(&self) -> Link<T> {
        unsafe { (*self.inner.get()).next }
    }

    pub fn prev(&self) -> Link<T> {
        unsafe { (*self.inner.get()).prev }
    }

    pub(crate) fn set_next(&mut self, val: Link<T>) -> Link<T> {
        mem::replace(&mut self.inner.get_mut().next, val)
    }

    pub(crate) fn set_prev(&mut self, val: Link<T>) -> Link<T> {
        mem::replace(&mut self.inner.get_mut().prev, val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr::NonNull;
    use proptest::prelude::*;

    struct Node {
        links: Links<Node>,
    }

    fn make_ptrs(n: usize) -> Vec<Node> {
        (0..n).map(|_| Node { links: Links::new() }).collect()
    }

    proptest! {
        #![proptest_config(proptest::test_runner::Config {
            failure_persistence: None,
            ..Default::default()
        })]

        #[test]
        fn set_next_returns_old_and_stores_new(indices in proptest::collection::vec(0usize..8, 1..16usize)) {
            let mut nodes: Vec<Node> = make_ptrs(8);
            let ptrs: Vec<NonNull<Node>> = nodes.iter_mut().map(NonNull::from).collect();
            let mut links: Links<Node> = Links::new();
            let mut expected: Link<Node> = None;
            for &i in &indices {
                let new_ptr: NonNull<Node> = ptrs[i];
                let old: Link<Node> = links.set_next(Some(new_ptr));
                prop_assert_eq!(old, expected);
                expected = Some(new_ptr);
            }
            prop_assert_eq!(links.next(), expected);
        }

        #[test]
        fn set_prev_returns_old_and_stores_new(indices in proptest::collection::vec(0usize..8, 1..16usize)) {
            let mut nodes: Vec<Node> = make_ptrs(8);
            let ptrs: Vec<NonNull<Node>> = nodes.iter_mut().map(NonNull::from).collect();
            let mut links: Links<Node> = Links::new();
            let mut expected: Link<Node> = None;
            for &i in &indices {
                let new_ptr: NonNull<Node> = ptrs[i];
                let old: Link<Node> = links.set_prev(Some(new_ptr));
                prop_assert_eq!(old, expected);
                expected = Some(new_ptr);
            }
            prop_assert_eq!(links.prev(), expected);
        }

    }
}
