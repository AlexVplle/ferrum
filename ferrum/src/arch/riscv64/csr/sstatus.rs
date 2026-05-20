use ferrum_macros::flag;

csr!(Sstatus, 0x8000_0000_000d_e762, 0x100);

impl Sstatus {
    flag!(supervisor_interrupt_enable, 1);
    flag!(supervisor_previous_interrupt_enable, 5);
    flag!(supervisor_previous_privilege, 8);
}
