static mut _TOTALRAM_PAGES: usize = 0;
static mut _TOTALRESERVE_PAGES: usize = 0;

pub fn totalram_pages() -> usize {
    unsafe { _TOTALRAM_PAGES }
}

pub fn adjust_totalram_page_count(delta: isize) {
    unsafe {
        _TOTALRAM_PAGES = _TOTALRAM_PAGES.saturating_add_signed(delta);
    }
}

pub fn totalreserve_pages() -> usize {
    unsafe { _TOTALRESERVE_PAGES }
}

pub fn set_totalreserve_pages(value: usize) {
    unsafe {
        _TOTALRESERVE_PAGES = value;
    }
}
