use ferrum_macros::flag;

#[derive(Clone, Copy, Default)]
pub struct MemorySectionFlags(usize);

impl MemorySectionFlags {
    pub const fn new() -> Self {
        Self(0)
    }

    flag!(marked_present, 0);
    flag!(has_memory_map, 1);
    flag!(is_online, 2);
    flag!(is_early, 3);
    flag!(taint_zone_device, 4);
}
