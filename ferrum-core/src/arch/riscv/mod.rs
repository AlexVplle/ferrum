use core::fmt;

pub fn console_write(args: fmt::Arguments) {
    use core::fmt::Write;
    sbi::debug_console::DebugConsoleWriter.write_fmt(args).ok();
}

pub fn current_processor_id() -> usize {
    let thread_info: *const crate::thread_info::ThreadInfo;
    unsafe {
        core::arch::asm!("mv {}, tp", out(reg) thread_info);
        (*thread_info).processor_id
    }
}

pub fn current_task_id() -> u64 {
    let thread_info: *const crate::thread_info::ThreadInfo;
    unsafe {
        core::arch::asm!("mv {}, tp", out(reg) thread_info);
        (*thread_info).task_id
    }
}
