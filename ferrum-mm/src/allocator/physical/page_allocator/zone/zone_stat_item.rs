#[derive(Clone, Copy)]
#[repr(usize)]
pub enum ZoneStatItem {
    FreePages,
    ZoneInactiveAnon,
    ZoneActiveAnon,
    ZoneInactiveFile,
    ZoneActiveFile,
    ZoneUnevictable,
    ZoneWritePending,
    Mlock,
    Bounce,
    FreeCmaPages,
}

pub const NR_VIRTUAL_MEMORY_ZONE_STAT_ITEMS: usize = 10;
