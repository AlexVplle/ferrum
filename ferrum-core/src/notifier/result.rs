use super::constants::STOP_MASK;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum NotifierResult {
    Done = 0x0000,
    Ok   = 0x0001,
    Bad  = STOP_MASK | 0x0002,
    Stop = STOP_MASK | 0x0001,
}

impl NotifierResult {
    pub fn should_stop(self) -> bool {
        (self as usize) & STOP_MASK != 0
    }
}
