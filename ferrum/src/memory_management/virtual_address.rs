use ferrum_macros::AddressFunctions;

#[derive(Clone, Copy, AddressFunctions)]
#[repr(transparent)]
pub struct VirtualAddress(usize);
