use super::cpumask_iter::CpuMaskIter;

#[derive(Clone, Copy)]
pub struct CpuMask(usize);

impl CpuMask {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn set(&mut self, cpu: usize) {
        self.0 |= 1 << cpu;
    }

    pub fn clear(&mut self, cpu: usize) {
        self.0 &= !(1 << cpu);
    }

    pub fn is_set(&self, cpu: usize) -> bool {
        self.0 & (1 << cpu) != 0
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn iter(&self) -> CpuMaskIter {
        CpuMaskIter::new(self.0)
    }
}

impl From<usize> for CpuMask {
    fn from(bits: usize) -> Self {
        Self(bits)
    }
}

impl core::fmt::Display for CpuMask {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#b}", self.0)
    }
}

impl IntoIterator for CpuMask {
    type Item = usize;
    type IntoIter = CpuMaskIter;

    fn into_iter(self) -> Self::IntoIter {
        CpuMaskIter::new(self.0)
    }
}
