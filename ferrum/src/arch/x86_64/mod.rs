use crate::boot_info::{BootInfo, Framebuffer};
use crate::limine::framebuffer::framebuffer::LimineFramebuffer;
use crate::limine::framebuffer::request::FRAMEBUFFER_REQUEST;

#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    let framebuffer: Option<Framebuffer> = match FRAMEBUFFER_REQUEST.get_response() {
        None => None,
        Some(resp) => {
            if resp.data.framebuffer_count == 0 {
                None
            } else {
                let fb: &LimineFramebuffer = unsafe { &**resp.data.framebuffers };
                let pixels_per_scan_line: usize = fb.pitch as usize / (fb.bpp as usize / 8);
                Some(Framebuffer {
                    address: fb.address as *mut u32,
                    width: fb.width as usize,
                    height: fb.height as usize,
                    pixels_per_scan_line,
                })
            }
        }
    };

    crate::kernel_main(BootInfo { framebuffer });
}
