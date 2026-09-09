use ferrum_macros::flag;

#[derive(Clone, Copy)]
pub struct MemoryBlockRegionFlags(usize);

impl MemoryBlockRegionFlags {
    pub const fn new() -> Self {
        Self(0)
    }

    flag!(hotplug, 0);
    flag!(mirror, 1);
    flag!(no_map, 2);
    flag!(driver_managed, 3);
    flag!(rsrv_noinit, 4);
    flag!(rsrv_kern, 5);
    flag!(kho_scratch, 6);
}
