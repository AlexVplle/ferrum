pub const SUPERVISOR_SOFTWARE_INTERRUPT_PENDING: usize = 1 << 1;

csr!(Sip, 0x222, 0x144);
