pub const LOCAL_DISTANCE: u8 = 10;
pub const REMOTE_DISTANCE: u8 = 20;

pub const DISTANCE_MATRIX_ENTRY_FIELDS: usize = 3;
pub const DISTANCE_MATRIX_ENTRY_SIZE: usize =
    DISTANCE_MATRIX_ENTRY_FIELDS * core::mem::size_of::<u32>();
