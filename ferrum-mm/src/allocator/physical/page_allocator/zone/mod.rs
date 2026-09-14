pub mod zone_stat_item;

use super::constants::NR_ZONES;
use super::watermark::{NR_WATERMARKS, Watermark};
use super::zone_type::ZoneType;
use super::super::buddy::BuddyAllocator;
use core::sync::atomic::{AtomicUsize, Ordering};
use zone_stat_item::{NR_VIRTUAL_MEMORY_ZONE_STAT_ITEMS, ZoneStatItem};

pub struct Zone {
    pub buddy: BuddyAllocator,
    zone_type: ZoneType,
    present_pages: usize,
    managed_pages: usize,
    watermarks: [usize; NR_WATERMARKS],
    virtual_memory_stat: [AtomicUsize; NR_VIRTUAL_MEMORY_ZONE_STAT_ITEMS],
    pub(super) low_memory_reserve: [usize; NR_ZONES],
}

impl Zone {
    pub const fn empty() -> Self {
        const ZERO: AtomicUsize = AtomicUsize::new(0);
        Self {
            buddy: BuddyAllocator::empty(),
            zone_type: ZoneType::Normal,
            present_pages: 0,
            managed_pages: 0,
            watermarks: [0; NR_WATERMARKS],
            virtual_memory_stat: [ZERO; NR_VIRTUAL_MEMORY_ZONE_STAT_ITEMS],
            low_memory_reserve: [0; NR_ZONES],
        }
    }

    pub fn zone_type(&self) -> ZoneType {
        self.zone_type
    }

    pub fn present_pages(&self) -> usize {
        self.present_pages
    }

    pub fn managed_pages(&self) -> usize {
        self.managed_pages
    }

    pub fn watermark(&self, watermark: Watermark) -> usize {
        self.watermarks[watermark as usize]
    }

    pub fn set_watermark(&mut self, watermark: Watermark, value: usize) {
        self.watermarks[watermark as usize] = value;
    }

    pub fn high_watermark_pages(&self) -> usize {
        self.watermarks[Watermark::High as usize]
    }

    pub fn zone_page_state(&self, item: ZoneStatItem) -> usize {
        self.virtual_memory_stat[item as usize].load(Ordering::Relaxed)
    }

    pub fn mod_zone_page_state(&self, item: ZoneStatItem, delta: isize) {
        self.virtual_memory_stat[item as usize].fetch_update(
            Ordering::Relaxed,
            Ordering::Relaxed,
            |value: usize| Some(value.saturating_add_signed(delta)),
        ).ok();
    }

    pub fn adjust_managed_page_count(&mut self, delta: isize) {
        self.managed_pages = self.managed_pages.saturating_add_signed(delta);
        crate::total_ram_pages::adjust_totalram_page_count(delta);
    }

    pub fn low_memory_reserve(&self, zone_index: usize) -> usize {
        self.low_memory_reserve[zone_index]
    }

    pub fn populated_zone(&self) -> bool {
        self.present_pages > 0
    }

    pub fn init(&mut self, zone_type: ZoneType, base: crate::physical_address::PhysicalAddress, num_pages: usize) {
        self.zone_type = zone_type;
        self.present_pages += num_pages;
        self.buddy.init(base, num_pages);
        self.adjust_managed_page_count(num_pages as isize);
        self.mod_zone_page_state(ZoneStatItem::FreePages, num_pages as isize);
    }
}
