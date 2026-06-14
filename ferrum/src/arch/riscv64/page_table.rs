use super::constants::PAGE_TABLE_ENTRIES;
use super::page_table_entry::PageTableEntry;

#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; PAGE_TABLE_ENTRIES],
}

impl PageTable {
    pub const fn empty() -> Self {
        Self {
            entries: [PageTableEntry::empty(); PAGE_TABLE_ENTRIES],
        }
    }

    pub fn entry(&self, index: usize) -> &PageTableEntry {
        &self.entries[index]
    }

    pub fn entry_mut(&mut self, index: usize) -> &mut PageTableEntry {
        &mut self.entries[index]
    }

    pub unsafe fn write_entry(this: *mut Self, index: usize, entry: PageTableEntry) {
        unsafe {
            let entries_ptr: *mut PageTableEntry =
                core::ptr::addr_of_mut!((*this).entries) as *mut PageTableEntry;
            core::ptr::write(entries_ptr.add(index), entry);
        }
    }
}
