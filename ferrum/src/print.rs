use core::fmt;

pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    sbi::debug_console::DebugConsoleWriter.write_fmt(args).ok();
}

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => { $crate::print::_print(format_args!($($arg)*)) };
}

#[macro_export]
macro_rules! printkln {
    () => { $crate::printk!("\n") };
    ($($arg:tt)*) => { $crate::printk!("{}\n", format_args!($($arg)*)) };
}
