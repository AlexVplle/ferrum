use super::error::MemoryError;
use super::page_flags::PageFlags;
use super::physical_address::PhysicalAddress;
use super::virtual_address::VirtualAddress;

pub struct Page {
    physical_address: PhysicalAddress,
    flags: PageFlags,
}

impl Page {
    pub fn new(physical_address: PhysicalAddress) -> Result<Self, MemoryError> {
        if !physical_address.is_aligned(core::ptr::Alignment::of::<[u8; 4096]>()) {
            return Err(MemoryError::UnalignedAddress);
        }
        Ok(Self {
            physical_address,
            flags: PageFlags::new(),
        })
    }

    pub fn physical_address(&self) -> &PhysicalAddress {
        &self.physical_address
    }

    pub fn flags(&self) -> &PageFlags {
        &self.flags
    }

    pub fn flags_mut(&mut self) -> &mut PageFlags {
        &mut self.flags
    }
}
