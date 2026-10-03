pub mod numa_event_item;
pub mod zone_stat_item;

use super::alloc_flags::AllocFlags;
use super::constants::{NR_ZONES, WATERMARK_BOOST_FACTOR};
use super::watermark::{NR_WATERMARKS, Watermark};
use super::zone_type::ZoneType;
use super::per_cpu_pages::PerCpuPages;
use super::per_cpu_zonestat::PerCpuZoneStat;
use super::super::buddy::{BuddyAllocator, MAX_PAGE_ORDER};
use super::super::buddy::constants::PAGEBLOCK_NR_PAGES;
use crate::migrate_type::MigrateType;
use core::sync::atomic::{AtomicUsize, Ordering};
use ferrum_core::constants::MAX_CPUS;
use ferrum_core::per_cpu::PerCpu;
use ferrum_core::spinlock::{Spinlock, SpinlockGuard};
use lock_dependency::LockClassKey;
use numa_event_item::{NR_NUMA_EVENT_ITEMS, NumaEventItem};
use zone_stat_item::{NR_VIRTUAL_MEMORY_ZONE_STAT_ITEMS, ZoneStatItem};

static ZONE_BUDDY_KEY: LockClassKey = LockClassKey::new();

pub struct Zone {
    pub buddy: Spinlock<BuddyAllocator>,
    zone_type: ZoneType,
    node_id: usize,
    start_page_frame_number: usize,
    present_pages: usize,
    managed_pages: usize,
    pub contiguous: bool,
    watermarks: [usize; NR_WATERMARKS],
    watermark_boost: usize,
    nr_reserved_highatomic: usize,
    virtual_memory_stat: [AtomicUsize; NR_VIRTUAL_MEMORY_ZONE_STAT_ITEMS],
    numa_event: [AtomicUsize; NR_NUMA_EVENT_ITEMS],
    pub(super) low_memory_reserve: [usize; NR_ZONES],
    pub pageset: PerCpu<PerCpuPages, MAX_CPUS>,
    pub pageset_high: usize,
    pub pageset_batch: usize,
    pub per_cpu_zonestats: PerCpu<PerCpuZoneStat, MAX_CPUS>,
}

impl Zone {
    pub const fn empty() -> Self {
        const ZERO: AtomicUsize = AtomicUsize::new(0);
        Self {
            buddy: Spinlock::new_tracked(BuddyAllocator::empty(), &ZONE_BUDDY_KEY, "zone_buddy"),
            zone_type: ZoneType::Normal,
            node_id: 0,
            start_page_frame_number: 0,
            present_pages: 0,
            managed_pages: 0,
            contiguous: false,
            watermarks: [0; NR_WATERMARKS],
            watermark_boost: 0,
            nr_reserved_highatomic: 0,
            virtual_memory_stat: [ZERO; NR_VIRTUAL_MEMORY_ZONE_STAT_ITEMS],
            numa_event: [ZERO; NR_NUMA_EVENT_ITEMS],
            low_memory_reserve: [0; NR_ZONES],
            pageset: PerCpu::new(),
            pageset_high: 0,
            pageset_batch: 1,
            per_cpu_zonestats: PerCpu::new(),
        }
    }

    pub fn lock_buddy(&self) -> SpinlockGuard<'_, BuddyAllocator> {
        self.buddy.lock()
    }

    pub fn zone_type(&self) -> ZoneType {
        self.zone_type
    }

    pub fn node_id(&self) -> usize {
        self.node_id
    }

    pub fn start_page_frame_number(&self) -> usize {
        self.start_page_frame_number
    }

    pub fn present_pages(&self) -> usize {
        self.present_pages
    }

    pub fn managed_pages(&self) -> usize {
        self.managed_pages
    }

    pub fn watermark(&self, watermark: Watermark) -> usize {
        self.watermarks[watermark as usize] + self.watermark_boost
    }

    pub fn set_watermark(&mut self, watermark: Watermark, value: usize) {
        self.watermarks[watermark as usize] = value;
    }

    pub fn minimum_watermark_pages(&self) -> usize {
        self.watermark(Watermark::Minimum)
    }

    pub fn low_watermark_pages(&self) -> usize {
        self.watermark(Watermark::Low)
    }

    pub fn high_watermark_pages(&self) -> usize {
        self.watermarks[Watermark::High as usize]
    }

    pub fn watermark_boost(&self) -> usize {
        self.watermark_boost
    }

    pub fn set_watermark_boost(&mut self, value: usize) {
        self.watermark_boost = value;
    }

    pub fn boost_watermark(&mut self) -> bool {
        if WATERMARK_BOOST_FACTOR == 0 {
            return false;
        }
        if (PAGEBLOCK_NR_PAGES * 4) > self.managed_pages {
            return false;
        }
        let max_boost: usize = ((self.watermarks[Watermark::High as usize] as u128
            * WATERMARK_BOOST_FACTOR as u128
            / 10000) as usize)
            .max(PAGEBLOCK_NR_PAGES);
        self.watermark_boost = (self.watermark_boost + PAGEBLOCK_NR_PAGES).min(max_boost);
        true
    }

    pub fn nr_reserved_highatomic(&self) -> usize {
        self.nr_reserved_highatomic
    }

    pub fn set_nr_reserved_highatomic(&mut self, value: usize) {
        self.nr_reserved_highatomic = value;
    }

    pub fn zone_page_state(&self, item: ZoneStatItem) -> usize {
        self.virtual_memory_stat[item as usize].load(Ordering::Relaxed)
    }

    pub fn numa_event(&self, item: NumaEventItem) -> usize {
        self.numa_event[item as usize].load(Ordering::Relaxed)
    }

    pub fn increase_numa_event(&self, item: NumaEventItem) {
        self.numa_event[item as usize].fetch_add(1, Ordering::Relaxed);
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

    pub fn zone_watermark_ok(
        &self,
        order: usize,
        mark: usize,
        highest_zoneidx: usize,
        alloc_flags: AllocFlags,
        free_pages: usize,
    ) -> bool {
        let alloc_harder: bool = alloc_flags.contains(AllocFlags::HARDER)
            || alloc_flags.contains(AllocFlags::OUT_OF_MEMORY);

        let mut free_pages: isize = free_pages as isize - ((1usize << order) - 1) as isize;
        let mut min: isize = mark as isize;

        if alloc_flags.contains(AllocFlags::HIGH) {
            min -= min / 2;
        }

        if !alloc_harder {
            free_pages -= self.nr_reserved_highatomic as isize;
        } else if alloc_flags.contains(AllocFlags::OUT_OF_MEMORY) {
            min -= min / 2;
        } else {
            min -= min / 4;
        }

        if free_pages <= min + self.low_memory_reserve[highest_zoneidx] as isize {
            return false;
        }

        let buddy: ferrum_core::spinlock::SpinlockGuard<'_, BuddyAllocator> = self.buddy.lock();
        for o in order..=MAX_PAGE_ORDER {
            for migrate_type in [MigrateType::Unmovable, MigrateType::Movable, MigrateType::Reclaimable] {
                if !buddy.is_area_empty(o, migrate_type) {
                    return true;
                }
            }
            if alloc_harder && !buddy.is_area_empty(o, MigrateType::HighAtomic) {
                return true;
            }
        }
        false
    }

    pub fn populated_zone(&self) -> bool {
        self.present_pages > 0
    }

    pub fn init(&mut self, zone_type: ZoneType, node_id: usize, base: crate::physical_address::PhysicalAddress, num_pages: usize) {
        self.zone_type = zone_type;
        self.node_id = node_id;
        self.start_page_frame_number = base.to_page_frame_number();
        self.present_pages += num_pages;
        self.buddy.lock().init(base, num_pages);
        self.adjust_managed_page_count(num_pages as isize);
        self.mod_zone_page_state(ZoneStatItem::FreePages, num_pages as isize);
    }
}
