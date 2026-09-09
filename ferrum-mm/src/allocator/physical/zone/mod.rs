pub mod allocator;
pub mod constants;

pub use constants::DIRECT_MEMORY_ACCESS_ZONE_END;

#[derive(Clone, Copy)]
pub enum Zone {
    DirectMemoryAccess,
    Normal,
    Device,
}
