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

    pub fn present_pages(&self) -> usize {
        self.buddy.total_pages()
    }
}
