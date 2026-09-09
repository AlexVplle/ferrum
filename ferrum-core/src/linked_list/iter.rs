use core::marker::PhantomData;
use core::ptr::NonNull;

use super::link::Link;
use super::linked::Linked;
use super::links::Links;

pub struct Iter<'a, T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iter<'a, T> {
    pub(super) fn new(head: Link<T>, tail: Link<T>, len: usize) -> Self {
        Self {
            head,
            tail,
            len,
            _marker: PhantomData,
        }
    }
}

impl<'a, T: Linked<Links<T>>> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let ptr: NonNull<T> = self.head?;
        let node: &'a T = unsafe { ptr.as_ref() };
        self.len -= 1;
        if self.len == 0 {
            self.head = None;
            self.tail = None;
        } else {
            self.head = unsafe { T::links(ptr).as_ref() }.next();
        }
        Some(node)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T: Linked<Links<T>>> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let ptr: NonNull<T> = self.tail?;
        let node: &'a T = unsafe { ptr.as_ref() };
        self.len -= 1;
        if self.len == 0 {
            self.head = None;
            self.tail = None;
        } else {
            self.tail = unsafe { T::links(ptr).as_ref() }.prev();
        }
        Some(node)
    }
}

impl<'a, T: Linked<Links<T>>> ExactSizeIterator for Iter<'a, T> {}

impl<'a, T: Linked<Links<T>>> core::iter::FusedIterator for Iter<'a, T> {}
