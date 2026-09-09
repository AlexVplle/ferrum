use core::ptr::NonNull;

use crate::arch::PAGE_SIZE;
use crate::physical_address::PhysicalAddress;
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

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr::NonNull;
    use proptest::prelude::*;

    const BASE: usize = 0x1000_0000;

    fn make_allocator(bitmap: &mut Vec<u64>, page_count: usize) -> BitmapAllocator {
        let words: usize = (page_count + u64::BITS as usize - 1) / u64::BITS as usize;
        bitmap.resize(words, 0);
        let ptr: NonNull<u64> = NonNull::new(bitmap.as_mut_ptr()).unwrap();
        BitmapAllocator::new(ptr, words, BASE, page_count)
    }

    proptest! {
        #![proptest_config(proptest::test_runner::Config {
            failure_persistence: None,
            ..Default::default()
        })]

        #[test]
        fn alloc_returns_addresses_in_range(page_count in 1usize..256) {
            let mut bitmap: Vec<u64> = Vec::new();
            let mut alloc: BitmapAllocator = make_allocator(&mut bitmap, page_count);
            while let Some(addr) = alloc.alloc(PAGE_SIZE, 0) {
                prop_assert!(addr.as_usize() >= BASE);
                prop_assert!(addr.as_usize() < BASE + page_count * PAGE_SIZE);
                prop_assert_eq!((addr.as_usize() - BASE) % PAGE_SIZE, 0);
            }
        }

        #[test]
        fn alloc_exhausts_exactly_page_count(page_count in 1usize..256) {
            let mut bitmap: Vec<u64> = Vec::new();
            let mut alloc: BitmapAllocator = make_allocator(&mut bitmap, page_count);
            let mut count: usize = 0;
            while alloc.alloc(PAGE_SIZE, 0).is_some() {
                count += 1;
            }
            prop_assert_eq!(count, page_count);
        }

        #[test]
        fn no_duplicate_allocations(page_count in 1usize..256) {
            let mut bitmap: Vec<u64> = Vec::new();
            let mut alloc: BitmapAllocator = make_allocator(&mut bitmap, page_count);
            let mut addrs: std::collections::HashSet<usize> = std::collections::HashSet::new();
            while let Some(addr) = alloc.alloc(PAGE_SIZE, 0) {
                prop_assert!(addrs.insert(addr.as_usize()), "duplicate address: {:#x}", addr.as_usize());
            }
        }

        #[test]
        fn free_makes_page_available_again(page_count in 2usize..256) {
            let mut bitmap: Vec<u64> = Vec::new();
            let mut alloc: BitmapAllocator = make_allocator(&mut bitmap, page_count);
            let first: PhysicalAddress = alloc.alloc(PAGE_SIZE, 0).unwrap();
            alloc.free(first, PAGE_SIZE);
            let realloced: PhysicalAddress = alloc.alloc(PAGE_SIZE, 0).unwrap();
            prop_assert_eq!(first.as_usize(), realloced.as_usize());
        }

        #[test]
        fn alloc_free_roundtrip(page_count in 1usize..64) {
            let mut bitmap: Vec<u64> = Vec::new();
            let mut alloc: BitmapAllocator = make_allocator(&mut bitmap, page_count);
            let mut addrs: Vec<PhysicalAddress> = Vec::new();
            while let Some(addr) = alloc.alloc(PAGE_SIZE, 0) {
                addrs.push(addr);
            }
            prop_assert_eq!(addrs.len(), page_count);
            for addr in &addrs {
                alloc.free(*addr, PAGE_SIZE);
            }
            let mut count: usize = 0;
            while alloc.alloc(PAGE_SIZE, 0).is_some() {
                count += 1;
            }
            prop_assert_eq!(count, page_count);
        }
    }
}
