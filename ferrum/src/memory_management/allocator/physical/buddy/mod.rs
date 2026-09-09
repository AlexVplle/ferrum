mod constants;
mod free_area;

pub use constants::MAX_ORDER;

use core::pin::Pin;
use free_area::FreeArea;
use crate::arch::PAGE_SIZE;
use crate::data_structures::list_head::ListHead;
use crate::memory_management::{
    page::{
        frame::Frame,
        frame_usage::FrameUsage,
        memory_section::{page_frame_number_to_page, page_frame_number_to_physical, physical_to_page_frame_number},
    },
    physical_address::PhysicalAddress,
    virtual_address::VirtualAddress,
};
use super::allocator::PhysicalAllocator;

pub struct BuddyAllocator {
    areas: [FreeArea; MAX_ORDER + 1],
    base_page_frame_number: usize,
    total_pages: usize,
}

impl BuddyAllocator {
    pub const fn empty() -> Self {
        const EMPTY_AREA: FreeArea = FreeArea::empty();
        Self {
            areas: [EMPTY_AREA; MAX_ORDER + 1],
            base_page_frame_number: 0,
            total_pages: 0,
        }
    }

    pub fn init(
        &mut self,
        base: PhysicalAddress,
        total_pages: usize,
        bitmap: core::ptr::NonNull<usize>,
    ) {
        self.base_page_frame_number = physical_to_page_frame_number(base.as_usize());
        self.total_pages = total_pages;

        let mut bitmap_offset: usize = 0;
        for order in 0..=MAX_ORDER {
            let bits: usize = (total_pages >> (order + 1)).max(1);
            let words: usize = (bits + usize::BITS as usize - 1) / usize::BITS as usize;
            unsafe {
                self.areas[order].map = bitmap.as_ptr().add(bitmap_offset);
                for i in 0..words {
                    bitmap.as_ptr().add(bitmap_offset + i).write(0);
                }
            }
            bitmap_offset += words;
        }

        let mut page_frame_number: usize = self.base_page_frame_number;
        let mut remaining: usize = total_pages;
        while remaining > 0 {
            let page_index: usize = page_frame_number - self.base_page_frame_number;
            let align_order: usize = (page_index.trailing_zeros() as usize).min(MAX_ORDER);
            let size_order: usize = (remaining.ilog2() as usize).min(MAX_ORDER);
            let order: usize = align_order.min(size_order);
            unsafe { self.push_block(page_frame_number, order) };
            page_frame_number += 1 << order;
            remaining -= 1 << order;
        }
    }

    pub fn bitmap_words_needed(total_pages: usize) -> usize {
        let mut total: usize = 0;
        for order in 0..=MAX_ORDER {
            let bits: usize = (total_pages >> (order + 1)).max(1);
            total += (bits + usize::BITS as usize - 1) / usize::BITS as usize;
        }
        total
    }

    unsafe fn push_block(&mut self, page_frame_number: usize, order: usize) {
        unsafe {
            let page: &mut Frame = &mut *page_frame_number_to_page(page_frame_number);
            page.usage = FrameUsage::Buddy { order };
            let node: *mut ListHead =
                PhysicalAddress::new(page_frame_number_to_physical(page_frame_number))
                    .to_virtual()
                    .as_usize() as *mut ListHead;
            self.areas[order].push_front(node);
        }
    }

    fn alloc_order(&mut self, order: usize) -> Option<usize> {
        if order > MAX_ORDER {
            return None;
        }
        if let Some(node) = self.areas[order].pop_front() {
            let page_frame_number: usize = physical_to_page_frame_number(
                VirtualAddress::new(node as usize).to_physical().as_usize(),
            );
            unsafe {
                let page: &mut Frame = &mut *page_frame_number_to_page(page_frame_number);
                page.usage = FrameUsage::Buddy { order: 0 };
            }
            return Some(page_frame_number);
        }
        let parent_page_frame_number: usize = self.alloc_order(order + 1)?;
        let buddy_page_frame_number: usize = parent_page_frame_number + (1 << order);
        unsafe { self.push_block(buddy_page_frame_number, order) };
        Some(parent_page_frame_number)
    }

    fn free_order(&mut self, page_frame_number: usize, order: usize) {
        if order >= MAX_ORDER {
            unsafe { self.push_block(page_frame_number, order) };
            return;
        }
        let page_index: usize = page_frame_number - self.base_page_frame_number;
        let buddy_page_index: usize = page_index ^ (1 << order);
        let buddy_page_frame_number: usize = self.base_page_frame_number + buddy_page_index;
        let bit_index: usize = page_index >> (order + 1);

        if buddy_page_frame_number < self.base_page_frame_number + self.total_pages {
            let should_merge: bool =
                unsafe { self.areas[order].toggle_and_test_buddy_bit(bit_index) };

            if should_merge {
                unsafe {
                    let buddy_page: &Frame = &*page_frame_number_to_page(buddy_page_frame_number);
                    if matches!(buddy_page.usage, FrameUsage::Buddy { order: o } if o == order) {
                        let buddy_node: *mut ListHead = PhysicalAddress::new(
                            page_frame_number_to_physical(buddy_page_frame_number),
                        )
                        .to_virtual()
                        .as_usize()
                            as *mut ListHead;
                        Pin::new_unchecked(&mut *buddy_node).remove();
                        let buddy_page_mut: &mut Frame =
                            &mut *page_frame_number_to_page(buddy_page_frame_number);
                        buddy_page_mut.usage = FrameUsage::Buddy { order: 0 };
                        let merged_page_frame_number: usize =
                            page_frame_number.min(buddy_page_frame_number);
                        self.free_order(merged_page_frame_number, order + 1);
                        return;
                    }
                }
            }
        }
        unsafe { self.push_block(page_frame_number, order) };
    }

    fn order_for_size(size: usize) -> usize {
        let pages: usize = (size + PAGE_SIZE - 1) / PAGE_SIZE;
        (pages.next_power_of_two().ilog2() as usize).min(MAX_ORDER)
    }
}

impl PhysicalAllocator for BuddyAllocator {
    fn alloc(&mut self, size: usize, _align: usize) -> Option<PhysicalAddress> {
        let order: usize = Self::order_for_size(size);
        self.alloc_order(order).map(|page_frame_number| {
            PhysicalAddress::new(unsafe { page_frame_number_to_physical(page_frame_number) })
        })
    }

    fn free(&mut self, address: PhysicalAddress, size: usize) {
        let order: usize = Self::order_for_size(size);
        self.free_order(physical_to_page_frame_number(address.as_usize()), order);
    }
}
