#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Watermark {
    Minimum,
    Low,
    High,
    Promo,
}

pub const NR_WATERMARKS: usize = 4;
