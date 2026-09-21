pub const MAX_NODES: usize = 16;
pub const MAX_ZONELISTS: usize = 2;
pub const NR_ZONES: usize = 4;
pub const MAX_ZONELIST_ENTRIES: usize = MAX_NODES * NR_ZONES;

pub const PAGE_ALLOC_COSTLY_ORDER: usize = 3;
pub const WATERMARK_BOOST_FACTOR: usize = 15000;
pub const NR_PCP_TRANSPARENT_HUGE_PAGES: usize = 0;
pub const NR_PER_CPU_PAGES_LISTS: usize =
    crate::migrate_type::NR_MOVABLE_MIGRATE_TYPES
    * (PAGE_ALLOC_COSTLY_ORDER + 1 + NR_PCP_TRANSPARENT_HUGE_PAGES);

pub const LOW_MEMORY_RESERVE_RATIO_DIRECT_MEMORY_ACCESS: usize = 256;
pub const LOW_MEMORY_RESERVE_RATIO_NORMAL: usize = 32;
pub const LOW_MEMORY_RESERVE_RATIO_MOVABLE: usize = 0;
pub const LOW_MEMORY_RESERVE_RATIO_DEVICE: usize = 0;
