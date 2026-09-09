pub struct CpuMaskIter {
    remaining: usize,
}

impl CpuMaskIter {
    pub fn new(bits: usize) -> Self {
        Self { remaining: bits }
    }
}

impl Iterator for CpuMaskIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let cpu: usize = self.remaining.trailing_zeros() as usize;
        self.remaining &= self.remaining - 1;
        Some(cpu)
    }
}

impl core::iter::FusedIterator for CpuMaskIter {}
