use crate::memory_management::virtual_address::VirtualAddress;
use crate::memory_management::virtual_memory_area::{VmaFlags, VirtualMemoryArea};

const MAX_VMAS: usize = 32;

pub struct MemoryDescriptor {
    vmas: [Option<VirtualMemoryArea>; MAX_VMAS],
    count: usize,
    pub page_table: usize,
}

impl MemoryDescriptor {
    pub const fn new(page_table: usize) -> Self {
        Self {
            vmas: [None; MAX_VMAS],
            count: 0,
            page_table,
        }
    }

    pub fn map(&mut self, start: VirtualAddress, end: VirtualAddress, flags: VmaFlags) -> bool {
        if start.as_usize() >= end.as_usize() {
            return false;
        }
        for slot in &self.vmas[..self.count] {
            if let Some(vma) = slot {
                if vma.overlaps(start, end) {
                    return false;
                }
            }
        }
        if self.count >= MAX_VMAS {
            return false;
        }
        self.vmas[self.count] = Some(VirtualMemoryArea::new(start, end, flags));
        self.count += 1;
        true
    }

    pub fn unmap(&mut self, start: VirtualAddress, end: VirtualAddress) {
        let unmap_start: usize = start.as_usize();
        let unmap_end: usize = end.as_usize();
        let mut tail: Option<VirtualMemoryArea> = None;

        for slot in &mut self.vmas[..self.count] {
            let Some(vma) = slot else { continue; };
            if !vma.overlaps(start, end) {
                continue;
            }
            let vma_start: usize = vma.start.as_usize();
            let vma_end: usize = vma.end.as_usize();

            if unmap_start <= vma_start && unmap_end >= vma_end {
                *slot = None;
            } else if unmap_start <= vma_start {
                vma.start = VirtualAddress::new(unmap_end);
            } else if unmap_end >= vma_end {
                vma.end = VirtualAddress::new(unmap_start);
            } else {
                tail = Some(VirtualMemoryArea::new(
                    VirtualAddress::new(unmap_end),
                    VirtualAddress::new(vma_end),
                    vma.flags,
                ));
                vma.end = VirtualAddress::new(unmap_start);
            }
        }

        if let Some(vma) = tail {
            if self.count < MAX_VMAS {
                self.vmas[self.count] = Some(vma);
                self.count += 1;
            }
        }

        self.vmas[..self.count].sort_unstable_by_key(|slot: &Option<VirtualMemoryArea>| {
            slot.map_or(usize::MAX, |vma: VirtualMemoryArea| vma.start.as_usize())
        });
        while self.count > 0 && self.vmas[self.count - 1].is_none() {
            self.count -= 1;
        }
    }

    pub fn find(&self, address: VirtualAddress) -> Option<&VirtualMemoryArea> {
        self.vmas[..self.count]
            .iter()
            .find_map(|slot: &Option<VirtualMemoryArea>| {
                slot.as_ref().filter(|vma: &&VirtualMemoryArea| vma.contains(address))
            })
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
