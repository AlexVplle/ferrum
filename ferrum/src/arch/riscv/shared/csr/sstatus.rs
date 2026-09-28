use ferrum_macros::flag;

pub const SUPERVISOR_PREVIOUS_PRIVILEGE_BIT: usize = 1 << 8;

csr!(Sstatus, crate::arch::SSTATUS_WRITE_MASK, 0x100);

impl Sstatus {
    flag!(supervisor_interrupt_enable, 1);
    flag!(supervisor_previous_interrupt_enable, 5);
    flag!(supervisor_previous_privilege, 8);
}
