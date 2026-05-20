use ferrum_macros::flag;

csr!(Sie, 0x222, 0x104);

impl Sie {
    flag!(supervisor_software_interrupt_enable, 1);
    flag!(supervisor_timer_interrupt_enable, 5);
    flag!(supervisor_external_interrupt_enable, 9);
}
