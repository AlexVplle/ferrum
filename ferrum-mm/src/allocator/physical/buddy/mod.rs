pub mod constants;
mod free_area;
mod free_block;

pub use constants::{MAX_PAGE_ORDER, NR_PAGE_ORDERS};

use super::allocator::PhysicalAllocator;
use crate::arch::PAGE_SIZE;
use crate::memory_block::MEMORY_BLOCK;
use crate::page::{
    frame::Frame, frame_usage::FrameUsage,
    section::{MEMORY_SECTION, memory_section::page_frame_number_to_physical},
};
use crate::physical_address::PhysicalAddress;
use crate::virtual_address::VirtualAddress;
use core::ptr::NonNull;
use free_area::FreeArea;
use free_block::FreeBlock;

pub struct BuddyAllocator {
    areas: [FreeArea; NR_PAGE_ORDERS],
    base_page_frame_number: usize,
    total_pages: usize,
}

impl BuddyAllocator {
    pub const fn empty() -> Self {
        const EMPTY_AREA: FreeArea = FreeArea::empty();
        Self {
            areas: [EMPTY_AREA; NR_PAGE_ORDERS],
            base_page_frame_number: 0,
            total_pages: 0,
        }
    }

    pub fn total_pages(&self) -> usize {
        self.total_pages
    }

    pub fn init(&mut self, base: PhysicalAddress, total_pages: usize) {
        self.base_page_frame_number = base.to_page_frame_number();
        self.total_pages = total_pages;
        self.init_bitmap();
        self.populate_free_lists();
    }

    fn populate_free_lists(&mut self) {
        let mut page_frame_number: usize = self.base_page_frame_number;
        let mut remaining: usize = self.total_pages;
        while remaining > 0 {
            let page_index: usize = page_frame_number - self.base_page_frame_number;
            let align_order: usize = (page_index.trailing_zeros() as usize).min(MAX_PAGE_ORDER);
            let size_order: usize = (remaining.ilog2() as usize).min(MAX_PAGE_ORDER);
            let order: usize = align_order.min(size_order);
            let block_size: usize = 1 << order;
            self.push_block(page_frame_number, order);
            page_frame_number += block_size;
            remaining -= block_size;
        }
    }

    fn init_bitmap(&mut self) {
        let mut words_per_order: [usize; NR_PAGE_ORDERS] = [0; NR_PAGE_ORDERS];
        let mut total_words: usize = 0;
        for order in 0..=MAX_PAGE_ORDER {
            let bits: usize = (self.total_pages >> (order + 1)).max(1);
            let words: usize = (bits + usize::BITS as usize - 1) / usize::BITS as usize;
            words_per_order[order] = words;
            total_words += words;
        }

        let bitmap: NonNull<usize> = unsafe {
            (*(&raw mut MEMORY_BLOCK)).alloc(
                total_words * core::mem::size_of::<usize>(),
                core::mem::align_of::<usize>(),
            )
        }
        .expect("BuddyAllocator::init: failed to allocate bitmap")
        .to_virtual()
        .as_non_null::<usize>();

        unsafe { core::ptr::write_bytes(bitmap.as_ptr(), 0, total_words) };

        let mut bitmap_offset: usize = 0;
        for order in 0..=MAX_PAGE_ORDER {
            unsafe { self.areas[order].map = bitmap.as_ptr().add(bitmap_offset) };
            bitmap_offset += words_per_order[order];
        }
    }

    fn push_block(&mut self, page_frame_number: usize, order: usize) {
        let page_index: usize = page_frame_number - self.base_page_frame_number;
        let bit_index: usize = page_index >> (order + 1);
        self.areas[order].toggle_buddy_bit(bit_index);
        unsafe {
            let page: &mut Frame = &mut *MEMORY_SECTION.page_frame_number_to_page(page_frame_number);
            page.set_buddy(order);
            let node: NonNull<FreeBlock> = NonNull::new_unchecked(
                PhysicalAddress::new(page_frame_number_to_physical(page_frame_number))
                    .to_virtual()
                    .as_usize() as *mut FreeBlock,
            );
            self.areas[order].push_front(node);
        }
    }

    fn alloc_order(&mut self, order: usize) -> Option<usize> {
        if order > MAX_PAGE_ORDER {
            return None;
        }
        if let Some(node) = self.areas[order].pop_front() {
            let page_frame_number: usize =
                VirtualAddress::new(node.as_ptr() as usize).to_page_frame_number();
            let page_index: usize = page_frame_number - self.base_page_frame_number;
            let bit_index: usize = page_index >> (order + 1);
            self.areas[order].toggle_buddy_bit(bit_index);
            unsafe {
                let page: &mut Frame =
                    &mut *MEMORY_SECTION.page_frame_number_to_page(page_frame_number);
                page.set_uninitialized();
            }
            return Some(page_frame_number);
        }
        let block_size: usize = 1 << order;
        let parent_page_frame_number: usize = self.alloc_order(order + 1)?;
        let buddy_page_frame_number: usize = parent_page_frame_number + block_size;
        self.push_block(buddy_page_frame_number, order);
        Some(parent_page_frame_number)
    }

    fn free_order(&mut self, page_frame_number: usize, order: usize) {
        debug_assert!(MEMORY_SECTION.page_frame_number_valid(page_frame_number));
        if order >= MAX_PAGE_ORDER {
            self.push_block(page_frame_number, order);
            return;
        }
        let block_size: usize = 1 << order;
        let page_index: usize = page_frame_number - self.base_page_frame_number;
        let buddy_page_index: usize = page_index ^ block_size;
        let buddy_page_frame_number: usize = self.base_page_frame_number + buddy_page_index;
        let bit_index: usize = page_index >> (order + 1);

        if buddy_page_frame_number + block_size <= self.base_page_frame_number + self.total_pages {
            let should_merge: bool = self.areas[order].test_buddy_bit(bit_index);

            if should_merge {
                unsafe {
                    let buddy_page: &Frame =
                        &*MEMORY_SECTION.page_frame_number_to_page(buddy_page_frame_number);
                    if matches!(buddy_page.get_usage(), FrameUsage::Buddy { order: o } if *o == order)
                    {
                        let buddy_node: NonNull<FreeBlock> = NonNull::new_unchecked(
                            PhysicalAddress::new(page_frame_number_to_physical(
                                buddy_page_frame_number,
                            ))
                            .to_virtual()
                            .as_usize() as *mut FreeBlock,
                        );
                        self.areas[order].remove(buddy_node);
                        self.areas[order].toggle_buddy_bit(bit_index);
                        let buddy_page_mut: &mut Frame =
                            &mut *MEMORY_SECTION.page_frame_number_to_page(buddy_page_frame_number);
                        buddy_page_mut.set_uninitialized();
                        let merged_page_frame_number: usize =
                            page_frame_number.min(buddy_page_frame_number);
                        self.free_order(merged_page_frame_number, order + 1);
                        return;
                    }
                }
            }
        }
        self.push_block(page_frame_number, order);
    }

    fn order_for_size(size: usize) -> usize {
        let pages: usize = (size + PAGE_SIZE - 1) / PAGE_SIZE;
        (pages.next_power_of_two().ilog2() as usize).min(MAX_PAGE_ORDER)
    }
}

impl PhysicalAllocator for BuddyAllocator {
    fn alloc(&mut self, size: usize, _align: usize) -> Option<PhysicalAddress> {
        let order: usize = Self::order_for_size(size);
        self.alloc_order(order).map(|page_frame_number| {
            PhysicalAddress::new(page_frame_number_to_physical(page_frame_number))
        })
    }

    fn free(&mut self, address: PhysicalAddress, size: usize) {
        let order: usize = Self::order_for_size(size);
        self.free_order(address.to_page_frame_number(), order);
    }
}
