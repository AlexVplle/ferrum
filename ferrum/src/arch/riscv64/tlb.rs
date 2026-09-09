pub fn flush_tlb_all() {
    unsafe {
        core::arch::asm!("sfence.vma x0, x0");
    }
}

pub fn flush_tlb_page(virtual_address: usize) {
    unsafe {
        core::arch::asm!("sfence.vma {0}, x0", in(reg) virtual_address);
    }
}

pub fn flush_tlb_kernel_range(start: usize, end: usize) {
    let pages: usize = (end - start) / super::constants::PAGE_SIZE;
    if pages > super::constants::TLB_FLUSH_ALL_THRESHOLD {
        flush_tlb_all();
        return;
    }
    let mut virtual_address: usize = start;
    while virtual_address < end {
        flush_tlb_page(virtual_address);
        virtual_address += super::constants::PAGE_SIZE;
    }
}
