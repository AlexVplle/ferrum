use ferrum_macros::flag;

use crate::memory_management::virtual_address::VirtualAddress;

#[derive(Copy, Clone)]
pub struct VmaFlags(usize);

impl VmaFlags {
    pub const fn new() -> Self {
        Self(0)
    }

    flag!(read, 0);
    flag!(write, 1);
    flag!(exec, 2);
    flag!(user, 3);
    flag!(shared, 4);

    pub const fn bits(&self) -> usize {
        self.0
    }
}

#[derive(Copy, Clone)]
pub struct VirtualMemoryArea {
    pub start: VirtualAddress,
    pub end: VirtualAddress,
    pub flags: VmaFlags,
}

impl VirtualMemoryArea {
    pub fn new(start: VirtualAddress, end: VirtualAddress, flags: VmaFlags) -> Self {
        Self { start, end, flags }
    }

    pub fn contains(&self, address: VirtualAddress) -> bool {
        address.as_usize() >= self.start.as_usize()
            && address.as_usize() < self.end.as_usize()
    }

    pub fn overlaps(&self, start: VirtualAddress, end: VirtualAddress) -> bool {
        self.start.as_usize() < end.as_usize()
            && self.end.as_usize() > start.as_usize()
    }
}
