#[derive(Clone, Copy)]
#[repr(usize)]
pub enum ZoneType {
    DirectMemoryAccess = 0,
    Normal = 1,
    Device = 2,
}
