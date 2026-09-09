use super::constants::SUBSECTIONS_PER_SECTION;

pub struct MemSectionUsage {
    pub subsection_map: usize,
}

impl MemSectionUsage {
    pub const fn new() -> Self {
        Self { subsection_map: 0 }
    }

    pub fn is_subsection_present(&self, index: usize) -> bool {
        debug_assert!(index < SUBSECTIONS_PER_SECTION);
        self.subsection_map & (1 << index) != 0
    }

    pub fn set_subsection_present(&mut self, index: usize) {
        debug_assert!(index < SUBSECTIONS_PER_SECTION);
        self.subsection_map |= 1 << index;
    }
}
