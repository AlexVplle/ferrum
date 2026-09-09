#[repr(C)]
pub struct LimineFlanTermFbInitParams {
    pub canvas: *const u32,
    pub canvas_size: u64,
    pub ansi_colours: [u32; 8],
    pub ansi_bright_colours: [u32; 8],
    pub default_bg: u32,
    pub default_fg: u32,
    pub default_bg_bright: u32,
    pub default_fg_bright: u32,
    pub font: *const u8,
    pub font_width: u64,
    pub font_height: u64,
    pub font_spacing: u64,
    pub font_scale_x: u64,
    pub font_scale_y: u64,
    pub margin: u64,
    pub rotation: u64,
}
