use crate::limine::framebuffer::framebuffer::LimineFramebuffer;

#[repr(C)]
pub struct LimineFramebufferResponse {
    pub framebuffer_count: u64,
    pub framebuffers: *const *const LimineFramebuffer,
}
