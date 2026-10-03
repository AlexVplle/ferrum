#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PagingMode {
    Sv39 = 8,
    Sv48 = 9,
    Sv57 = 10,
}

impl TryFrom<usize> for PagingMode {
    type Error = ();

    fn try_from(mode: usize) -> Result<Self, Self::Error> {
        match mode {
            8 => Ok(Self::Sv39),
            9 => Ok(Self::Sv48),
            10 => Ok(Self::Sv57),
            _ => Err(()),
        }
    }
}
