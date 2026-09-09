use core::ptr::NonNull;

use crate::arch::PAGE_SIZE;
use crate::memory_management::physical_address::PhysicalAddress;
use super::allocator::PhysicalAllocator;

pub struct BitmapAllocator {
    bitmap: NonNull<u64>,
    words: usize,
    base: usize,
}

impl BitmapAllocator {
    pub fn new(bitmap: NonNull<u64>, words: usize, base: usize, page_count: usize) -> Self {
        for i in 0..words {
            unsafe { bitmap.as_ptr().add(i).write(0) };
        }
        let used_bits: usize = page_count % u64::BITS as usize;
        if used_bits != 0 {
            let last_word: u64 = u64::MAX << used_bits;
            unsafe { bitmap.as_ptr().add(words - 1).write(last_word) };
        }
        Self { bitmap, words, base }
    }

    fn set(&mut self, index: usize) {
        unsafe {
            *self.bitmap.as_ptr().add(index / u64::BITS as usize) |=
                1u64 << (index % u64::BITS as usize)
        };
    }

    fn clear(&mut self, index: usize) {
        unsafe {
            *self.bitmap.as_ptr().add(index / u64::BITS as usize) &=
                !(1u64 << (index % u64::BITS as usize))
        };
    }
}

impl PhysicalAllocator for BitmapAllocator {
    fn alloc(&mut self, _size: usize, _align: usize) -> Option<PhysicalAddress> {
        for word_index in 0..self.words {
            let word: u64 = unsafe { *self.bitmap.as_ptr().add(word_index) };
            if word == u64::MAX {
                continue;
            }
            let bit: usize = word.trailing_ones() as usize;
            let index: usize = word_index * u64::BITS as usize + bit;
            self.set(index);
            return Some(PhysicalAddress::new(self.base + index * PAGE_SIZE));
        }
        None
    }

    fn free(&mut self, address: PhysicalAddress, _size: usize) {
        let index: usize = (address.as_usize() - self.base) / PAGE_SIZE;
        self.clear(index);
    }
}
