#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum MigrateType {
    Unmovable,
    Movable,
    Reclaimable,
    HighAtomic,
    ContiguousMemoryAllocator,
    Isolate,
}

pub const NR_MIGRATE_TYPES: usize = 6;
pub const NR_MOVABLE_MIGRATE_TYPES: usize = 3;

pub fn migratetype_is_mergeable(migrate_type: MigrateType) -> bool {
    (migrate_type as usize) < NR_MOVABLE_MIGRATE_TYPES
}
