use crate::migrate_type::{MigrateType, NR_MOVABLE_MIGRATE_TYPES};

pub const MAX_PAGE_ORDER: usize = 10;
pub const NR_PAGE_ORDERS: usize = MAX_PAGE_ORDER + 1;
pub const PAGEBLOCK_ORDER: usize = MAX_PAGE_ORDER;
pub const PAGEBLOCK_NR_PAGES: usize = 1 << PAGEBLOCK_ORDER;

pub const MIGRATE_FALLBACK: [[MigrateType; 2]; NR_MOVABLE_MIGRATE_TYPES] = [
    [MigrateType::Reclaimable, MigrateType::Movable],
    [MigrateType::Reclaimable, MigrateType::Unmovable],
    [MigrateType::Unmovable, MigrateType::Movable],
];
