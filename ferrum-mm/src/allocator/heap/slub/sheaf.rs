use core::ptr::NonNull;

use ferrum_core::linked_list::list::linked::Linked;
use ferrum_core::linked_list::list::links::Links;

use super::constants::SHEAF_CAPACITY;
use super::free_object::FreeObject;
use super::SlubCache;

pub struct SlabSheaf<const N: usize> {
    pub links: Links<SlabSheaf<N>>,
    pub cache: *const SlubCache<N>,
    pub size: usize,
    pub node: usize,
    pub objects: [Option<NonNull<FreeObject>>; SHEAF_CAPACITY],
}

unsafe impl<const N: usize> Send for SlabSheaf<N> {}

impl<const N: usize> SlabSheaf<N> {
    pub fn new(cache: *const SlubCache<N>, node: usize) -> Self {
        Self {
            links: Links::new(),
            cache,
            size: 0,
            node,
            objects: [const { None }; SHEAF_CAPACITY],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == SHEAF_CAPACITY
    }

    pub fn pop(&mut self) -> Option<NonNull<FreeObject>> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        self.objects[self.size].take()
    }

    pub fn push(&mut self, object: NonNull<FreeObject>) -> bool {
        if self.size == SHEAF_CAPACITY {
            return false;
        }
        self.objects[self.size] = Some(object);
        self.size += 1;
        true
    }
}

unsafe impl<const N: usize> Linked<Links<SlabSheaf<N>>> for SlabSheaf<N> {
    fn links(ptr: NonNull<SlabSheaf<N>>) -> NonNull<Links<SlabSheaf<N>>> {
        unsafe { NonNull::new_unchecked(&raw mut (*ptr.as_ptr()).links) }
    }
}
