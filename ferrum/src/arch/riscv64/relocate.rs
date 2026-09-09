pub fn rela_dyn_range() -> (usize, usize) {
    let rela_start: usize;
    let rela_end: usize;
    unsafe {
        core::arch::asm!(
            ".option push",
            ".option nopic",
            "la {start}, __rela_dyn_start",
            "la {end}, __rela_dyn_end",
            ".option pop",
            start = out(reg) rela_start,
            end = out(reg) rela_end,
        );
    }
    (rela_start, rela_end)
}
