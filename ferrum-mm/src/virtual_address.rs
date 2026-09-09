use super::page::frame::Frame;
use super::physical_address::PhysicalAddress;
use ferrum_macros::AddressFunctions;

#[derive(Clone, Copy, AddressFunctions)]
#[repr(transparent)]
pub struct VirtualAddress(usize);

impl VirtualAddress {
    pub fn to_physical(self) -> PhysicalAddress {
        PhysicalAddress::new(self.0.wrapping_sub(crate::arch::PAGE_OFFSET))
    }

    pub fn to_kernel_physical(self) -> PhysicalAddress {
        PhysicalAddress::new(self.0.wrapping_sub(crate::arch::PHYSICAL_TO_VIRTUAL_OFFSET))
    }

    pub fn to_page_frame_number(self) -> usize {
        self.to_physical().to_page_frame_number()
    }

    pub fn to_page(self) -> *mut Frame {
        let page_frame_number: usize = self.to_page_frame_number();
        crate::page::memory_section_table::MEM_SECTION.page_frame_number_to_page(page_frame_number)
    }
}
