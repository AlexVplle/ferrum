use super::zone::zone_stat_item::NR_VIRTUAL_MEMORY_ZONE_STAT_ITEMS;

pub struct PerCpuZoneStat {
    pub virtual_memory_stat_diff: [i8; NR_VIRTUAL_MEMORY_ZONE_STAT_ITEMS],
    pub stat_threshold: i8,
}

impl PerCpuZoneStat {
    pub const fn new() -> Self {
        Self {
            virtual_memory_stat_diff: [0; NR_VIRTUAL_MEMORY_ZONE_STAT_ITEMS],
            stat_threshold: 0,
        }
    }
}
