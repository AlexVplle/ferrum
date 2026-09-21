use super::constants::{
    LOW_MEMORY_RESERVE_RATIO_DEVICE, LOW_MEMORY_RESERVE_RATIO_DIRECT_MEMORY_ACCESS,
    LOW_MEMORY_RESERVE_RATIO_MOVABLE, LOW_MEMORY_RESERVE_RATIO_NORMAL,
};

#[derive(Clone, Copy)]
#[repr(usize)]
pub enum ZoneType {
    DirectMemoryAccess,
    Normal,
    Movable,
    Device,
}

impl ZoneType {
    pub const fn low_memory_reserve_ratio(self) -> usize {
        match self {
            ZoneType::DirectMemoryAccess => LOW_MEMORY_RESERVE_RATIO_DIRECT_MEMORY_ACCESS,
            ZoneType::Normal => LOW_MEMORY_RESERVE_RATIO_NORMAL,
            ZoneType::Movable => LOW_MEMORY_RESERVE_RATIO_MOVABLE,
            ZoneType::Device => LOW_MEMORY_RESERVE_RATIO_DEVICE,
        }
    }
}
