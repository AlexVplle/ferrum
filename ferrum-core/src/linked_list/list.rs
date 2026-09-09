use core::ptr::NonNull;

use super::link::Link;
use super::linked::Linked;
use super::links::Links;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

unsafe impl<T: Linked<Links<T>> + Send> Send for List<T> {}

impl<T: Linked<Links<T>>> List<T> {
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn front(&self) -> Link<T> {
        self.head
    }

    pub fn back(&self) -> Link<T> {
        self.tail
    }

    pub fn push_front(&mut self, handle: T::Handle) {
        let ptr: NonNull<T> = T::into_ptr(handle);
        let links: &mut Links<T> = unsafe { T::links(ptr).as_mut() };
        links.set_next(self.head);
        links.set_prev(None);
        match self.head {
            None => self.tail = Some(ptr),
            Some(old_head) => {
                let old_links: &mut Links<T> = unsafe { T::links(old_head).as_mut() };
                old_links.set_prev(Some(ptr));
            }
        }
        self.head = Some(ptr);
        self.len += 1;
    }

    pub fn push_back(&mut self, handle: T::Handle) {
        let ptr: NonNull<T> = T::into_ptr(handle);
        let links: &mut Links<T> = unsafe { T::links(ptr).as_mut() };
        links.set_prev(self.tail);
        links.set_next(None);
        match self.tail {
            None => self.head = Some(ptr),
            Some(old_tail) => {
                let old_links: &mut Links<T> = unsafe { T::links(old_tail).as_mut() };
                old_links.set_next(Some(ptr));
            }
        }
        self.tail = Some(ptr);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T::Handle> {
        let head_ptr: NonNull<T> = self.head?;
        let links: &mut Links<T> = unsafe { T::links(head_ptr).as_mut() };
        let next: Link<T> = links.next();
        links.set_next(None);
        match next {
            None => self.tail = None,
            Some(next_ptr) => {
                let next_links: &mut Links<T> = unsafe { T::links(next_ptr).as_mut() };
                next_links.set_prev(None);
            }
        }
        self.head = next;
        self.len -= 1;
        Some(T::from_ptr(head_ptr))
    }

    pub fn pop_back(&mut self) -> Option<T::Handle> {
        let tail_ptr: NonNull<T> = self.tail?;
        let links: &mut Links<T> = unsafe { T::links(tail_ptr).as_mut() };
        let prev: Link<T> = links.prev();
        links.set_prev(None);
        match prev {
            None => self.head = None,
            Some(prev_ptr) => {
                let prev_links: &mut Links<T> = unsafe { T::links(prev_ptr).as_mut() };
                prev_links.set_next(None);
            }
        }
        self.tail = prev;
        self.len -= 1;
        Some(T::from_ptr(tail_ptr))
    }

    pub unsafe fn remove(&mut self, handle: T::Handle) {
        let ptr: NonNull<T> = T::into_ptr(handle);
        let links: &mut Links<T> = unsafe { T::links(ptr).as_mut() };
        let prev: Link<T> = links.prev();
        let next: Link<T> = links.next();
        links.set_next(None);
        links.set_prev(None);
        match prev {
            Some(prev_ptr) => {
                let prev_links: &mut Links<T> = unsafe { T::links(prev_ptr).as_mut() };
                prev_links.set_next(next);
            }
            None => self.head = next,
        }
        match next {
            Some(next_ptr) => {
                let next_links: &mut Links<T> = unsafe { T::links(next_ptr).as_mut() };
                next_links.set_prev(prev);
            }
            None => self.tail = prev,
        }
        self.len -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    struct Node {
        links: Links<Node>,
        value: usize,
    }

    impl Node {
        fn new(value: usize) -> Self {
            Self {
                links: Links::new(),
                value,
            }
        }
    }

    unsafe impl Linked<Links<Node>> for Node {
        type Handle = NonNull<Node>;

        fn into_ptr(handle: NonNull<Node>) -> NonNull<Node> {
            handle
        }
        fn from_ptr(ptr: NonNull<Node>) -> NonNull<Node> {
            ptr
        }
        fn links(ptr: NonNull<Node>) -> NonNull<Links<Node>> {
            unsafe { NonNull::new_unchecked(&raw mut (*ptr.as_ptr()).links) }
        }
    }

    #[test]
    fn empty_list() {
        let list: List<Node> = List::new();
        assert!(list.is_empty());
        assert!(list.front().is_none());
        assert!(list.back().is_none());
    }

    #[test]
    fn push_front_and_front() {
        let mut list: List<Node> = List::new();
        let mut node: Node = Node::new(42);
        list.push_front(NonNull::from(&mut node));
        assert!(!list.is_empty());
        let front_value: usize = unsafe { list.front().unwrap().as_ref().value };
        assert_eq!(front_value, 42);
    }

    #[test]
    fn push_front_lifo_order() {
        let mut list: List<Node> = List::new();
        let mut first: Node = Node::new(1);
        let mut second: Node = Node::new(2);
        list.push_front(NonNull::from(&mut first));
        list.push_front(NonNull::from(&mut second));
        let front_value: usize = unsafe { list.front().unwrap().as_ref().value };
        assert_eq!(front_value, 2);
    }

    #[test]
    fn pop_front() {
        let mut list: List<Node> = List::new();
        let mut node: Node = Node::new(7);
        list.push_front(NonNull::from(&mut node));
        let popped_value: usize = unsafe { list.pop_front().unwrap().as_ref().value };
        assert_eq!(popped_value, 7);
        assert!(list.is_empty());
    }

    #[test]
    fn pop_front_empty() {
        let mut list: List<Node> = List::new();
        assert!(list.pop_front().is_none());
    }

    #[test]
    fn remove_node() {
        let mut list: List<Node> = List::new();
        let mut first: Node = Node::new(1);
        let mut second: Node = Node::new(2);
        list.push_front(NonNull::from(&mut first));
        list.push_front(NonNull::from(&mut second));
        unsafe { list.remove(NonNull::from(&mut first)) };
        let front_value: usize = unsafe { list.front().unwrap().as_ref().value };
        assert_eq!(front_value, 2);
        list.pop_front();
        assert!(list.is_empty());
    }

    #[test]
    fn len_tracks_operations() {
        let mut list: List<Node> = List::new();
        let mut a: Node = Node::new(1);
        let mut b: Node = Node::new(2);
        assert_eq!(list.len(), 0);
        list.push_front(NonNull::from(&mut a));
        assert_eq!(list.len(), 1);
        list.push_back(NonNull::from(&mut b));
        assert_eq!(list.len(), 2);
        list.pop_front();
        assert_eq!(list.len(), 1);
        list.pop_back();
        assert_eq!(list.len(), 0);
    }

    proptest! {
        #![proptest_config(proptest::test_runner::Config {
            failure_persistence: None,
            ..Default::default()
        })]

        #[test]
        fn push_pop_roundtrip(values in proptest::collection::vec(0usize..1000, 0..16usize)) {
            let mut nodes: Vec<Box<Node>> = values.iter().map(|&v| Box::new(Node::new(v))).collect();
            let mut list: List<Node> = List::new();
            for node in nodes.iter_mut() {
                list.push_front(NonNull::from(node.as_mut()));
            }
            let mut popped: Vec<usize> = Vec::new();
            while let Some(ptr) = list.pop_front() {
                popped.push(unsafe { ptr.as_ref().value });
            }
            let expected: Vec<usize> = values.iter().copied().rev().collect();
            prop_assert_eq!(popped, expected);
            prop_assert!(list.is_empty());
        }

        #[test]
        fn remove_decrements_count(
            values in proptest::collection::vec(0usize..1000, 1..9usize),
            remove_idx in 0usize..8,
        ) {
            let n: usize = values.len();
            let remove_i: usize = remove_idx % n;
            let mut nodes: Vec<Box<Node>> = values.iter().map(|&v| Box::new(Node::new(v))).collect();
            let mut list: List<Node> = List::new();
            for node in nodes.iter_mut() {
                list.push_front(NonNull::from(node.as_mut()));
            }
            unsafe { list.remove(NonNull::from(nodes[remove_i].as_mut())) };
            let mut count: usize = 0;
            while list.pop_front().is_some() { count += 1; }
            prop_assert_eq!(count, n - 1);
        }
    }
}
