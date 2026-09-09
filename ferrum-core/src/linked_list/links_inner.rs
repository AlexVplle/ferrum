use core::marker::PhantomPinned;

use super::link::Link;

pub(super) struct LinksInner<T> {
    pub(super) next: Link<T>,
    pub(super) prev: Link<T>,
    pub(super) _pin: PhantomPinned,
}

impl<T> LinksInner<T> {
    pub(super) const fn new() -> Self {
        Self {
            next: None,
            prev: None,
            _pin: PhantomPinned,
        }
    }
}
