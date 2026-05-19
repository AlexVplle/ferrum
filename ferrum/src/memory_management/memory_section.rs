use super::page::Page;

pub struct MemorySection {
    mem_map: *mut [Page],
    present: bool,
}

impl MemorySection {
    pub const fn empty() -> Self {
        Self {
            mem_map: core::ptr::slice_from_raw_parts_mut(core::ptr::null_mut(), 0),
            present: false,
        }
    }

    pub fn is_present(&self) -> bool {
        self.present
    }

    pub unsafe fn pages(&self) -> &[Page] {
        &*self.mem_map
    }

    pub unsafe fn pages_mut(&mut self) -> &mut [Page] {
        &mut *self.mem_map
    }
}
