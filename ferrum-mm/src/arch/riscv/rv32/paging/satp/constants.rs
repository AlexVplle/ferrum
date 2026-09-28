pub const CSR_ADDRESS: usize = 0x180;
pub const MODE_SHIFT: usize = 31;
pub const MODE_MASK: usize = 1 << MODE_SHIFT;
pub const MODE_SV32: usize = 1 << MODE_SHIFT;
pub const PPN_MASK: usize = (1 << MODE_SHIFT) - 1;
