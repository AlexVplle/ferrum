use ferrum_macros::flag;

pub const SUPERVISOR_SOFTWARE_INTERRUPT_ENABLE: usize = 1 << 1;
pub const SUPERVISOR_TIMER_INTERRUPT_ENABLE: usize = 1 << 5;
pub const SUPERVISOR_EXTERNAL_INTERRUPT_ENABLE: usize = 1 << 9;

csr!(Sie, 0x222, 0x104);

impl Sie {
    flag!(supervisor_software_interrupt_enable, 1);
    flag!(supervisor_timer_interrupt_enable, 5);
    flag!(supervisor_external_interrupt_enable, 9);
}
