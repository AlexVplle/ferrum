#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Watermark {
    Min,
    Low,
    High,
    Promo,
}

pub const NR_WATERMARKS: usize = 4;
