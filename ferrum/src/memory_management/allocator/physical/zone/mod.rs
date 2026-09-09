pub mod allocator;

pub const DIRECT_MEMORY_ACCESS_ZONE_END: usize = 0x1000000;

#[derive(Clone, Copy)]
pub enum Zone {
    DirectMemoryAccess,
    Normal,
    Device,
}
