use core::fmt;

pub fn console_write(args: fmt::Arguments) {
    use core::fmt::Write;
    sbi::debug_console::DebugConsoleWriter.write_fmt(args).ok();
}
