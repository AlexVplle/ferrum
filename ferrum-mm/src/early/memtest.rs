use crate::arch::PAGE_SIZE;
use crate::memory_block::{MemoryBlock, MemoryBlockRegion, MemoryBlockRegionFlags};
use crate::physical_address::PhysicalAddress;

const PATTERNS: [usize; 17] = [
    0x0000000000000000,
    0xffffffffffffffff,
    0x5555555555555555,
    0xaaaaaaaaaaaaaaaa,
    0x1111111111111111,
    0x2222222222222222,
    0x4444444444444444,
    0x8888888888888888,
    0x3333333333333333,
    0x6666666666666666,
    0x9999999999999999,
    0xcccccccccccccccc,
    0x7777777777777777,
    0xbbbbbbbbbbbbbbbb,
    0xdddddddddddddddd,
    0xeeeeeeeeeeeeeeee,
    0x7a6c7258554e494c,
];

const MAX_BAD_PAGES: usize = 64;

pub fn early_memtest(memory_block: &mut MemoryBlock) {
    let mut bad_pages: [PhysicalAddress; MAX_BAD_PAGES] = [PhysicalAddress::new(0); MAX_BAD_PAGES];
    let mut bad_count: usize = 0;

    memory_block.for_each_free_region(|base: PhysicalAddress, num_pages: usize, _node_id: usize| {
        for i in 0..num_pages {
            if bad_count >= MAX_BAD_PAGES {
                break;
            }
            let page: PhysicalAddress = PhysicalAddress::new(base.as_usize() + i * PAGE_SIZE);
            if !test_page(page) {
                ferrum_core::printkln!("[memtest] bad page at {}", page);
                bad_pages[bad_count] = page;
                bad_count += 1;
            }
        }
    });

    for i in 0..bad_count {
        memory_block.reserve(MemoryBlockRegion {
            base: bad_pages[i],
            size: PAGE_SIZE,
            flags: MemoryBlockRegionFlags::new(),
            node_id: 0,
        });
    }

    ferrum_core::printkln!("[memtest] {} bad page(s) found", bad_count);
}

fn test_page(page: PhysicalAddress) -> bool {
    let ptr: *mut usize = page.to_kernel_virtual().as_usize() as *mut usize;
    let count: usize = PAGE_SIZE / core::mem::size_of::<usize>();

    for &pattern in PATTERNS.iter() {
        for i in 0..count {
            unsafe { ptr.add(i).write_volatile(pattern) };
        }
        for i in 0..count {
            if unsafe { ptr.add(i).read_volatile() } != pattern {
                return false;
            }
        }
    }
    true
}
