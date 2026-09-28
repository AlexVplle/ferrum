pub const CSR_ADDRESS: usize = 0x180;
pub const MODE_SHIFT: usize = 60;
pub const MODE_MASK: usize = 0xF << MODE_SHIFT;
pub const MODE_SV39: usize = 8 << MODE_SHIFT;
pub const PPN_MASK: usize = (1 << MODE_SHIFT) - 1;
