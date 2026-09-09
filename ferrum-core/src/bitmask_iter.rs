pub struct BitmaskIter {
    remaining: usize,
}

impl BitmaskIter {
    pub fn new(bits: usize) -> Self {
        Self { remaining: bits }
    }
}

impl Iterator for BitmaskIter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.remaining == 0 {
            return None;
        }
        let bit: usize = self.remaining.trailing_zeros() as usize;
        self.remaining &= self.remaining - 1;
        Some(bit)
    }
}

impl core::iter::FusedIterator for BitmaskIter {}
