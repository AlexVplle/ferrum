use ferrum_macros::flag;

pub struct GetFreePageFlags(usize);

impl GetFreePageFlags {
    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn bits(&self) -> usize {
        self.0
    }

    pub const fn contains(&self, other: &Self) -> bool {
        self.0 & other.0 == other.0
    }

    flag!(direct_memory_access, 0);
    flag!(high_memory, 1);
    flag!(direct_memory_access_32, 2);
    flag!(movable, 3);
    flag!(reclaimable, 4);
    flag!(high_priority, 5);
    flag!(input_output, 6);
    flag!(file_system, 7);
    flag!(zero, 8);
    flag!(direct_reclaim, 9);
    flag!(kswapd_reclaim, 10);
    flag!(write, 11);
    flag!(no_warn, 12);
    flag!(retry_may_fail, 13);
    flag!(no_fail, 14);
    flag!(no_retry, 15);
    flag!(memory_allocation, 16);
    flag!(compound, 17);
    flag!(no_memory_allocation, 18);
    flag!(hard_wall, 19);
    flag!(this_node, 20);
    flag!(account, 21);
    flag!(zero_tags, 22);
}

impl core::ops::BitOr for GetFreePageFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

pub const GET_FREE_PAGE_ATOMIC: GetFreePageFlags = GetFreePageFlags::new()
    .high_priority()
    .kswapd_reclaim();
pub const GET_FREE_PAGE_KERNEL: GetFreePageFlags = GetFreePageFlags::new()
    .direct_reclaim()
    .kswapd_reclaim()
    .input_output()
    .file_system();
pub const GET_FREE_PAGE_KERNEL_ACCOUNT: GetFreePageFlags =
    GetFreePageFlags(GET_FREE_PAGE_KERNEL.0).account();
pub const GET_FREE_PAGE_NOWAIT: GetFreePageFlags = GetFreePageFlags::new().kswapd_reclaim();
pub const GET_FREE_PAGE_NOIO: GetFreePageFlags =
    GetFreePageFlags::new().direct_reclaim().kswapd_reclaim();
pub const GET_FREE_PAGE_NOFS: GetFreePageFlags = GetFreePageFlags::new()
    .direct_reclaim()
    .kswapd_reclaim()
    .input_output();
pub const GET_FREE_PAGE_USER: GetFreePageFlags = GetFreePageFlags::new()
    .direct_reclaim()
    .kswapd_reclaim()
    .input_output()
    .file_system()
    .hard_wall();
pub const GET_FREE_PAGE_DIRECT_MEMORY_ACCESS: GetFreePageFlags =
    GetFreePageFlags::new().direct_memory_access();
pub const GET_FREE_PAGE_DIRECT_MEMORY_ACCESS_32: GetFreePageFlags =
    GetFreePageFlags::new().direct_memory_access_32();
pub const GET_FREE_PAGE_HIGHUSER: GetFreePageFlags =
    GetFreePageFlags(GET_FREE_PAGE_USER.0).high_memory();
pub const GET_FREE_PAGE_HIGHUSER_MOVABLE: GetFreePageFlags =
    GetFreePageFlags(GET_FREE_PAGE_HIGHUSER.0).movable();
