use super::super::buddy::BuddyAllocator;

pub struct Zone {
    pub buddy: BuddyAllocator,
}

impl Zone {
    pub const fn empty() -> Self {
        Self {
            buddy: BuddyAllocator::empty(),
        }
    }

    pub fn init(&mut self, base: crate::physical_address::PhysicalAddress, num_pages: usize) {
        self.buddy.init(base, num_pages);
    }
}
