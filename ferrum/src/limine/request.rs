use core::cell::UnsafeCell;
use crate::limine::response::LimineResponse;

const COMMON_MAGIC: [u64; 2] = [0xc7b1dd30df4c8b88, 0x0a82e883a194f07b];

#[repr(C)]
pub struct LimineRequest<Resp, Req = ()> {
    common_magic: [u64; 2],
    id: [u64; 2],
    revision: u64,
    response: UnsafeCell<*mut LimineResponse<Resp>>,
    request: Req,
}

unsafe impl<Resp, Req> Sync for LimineRequest<Resp, Req> {}
unsafe impl<Resp, Req> Send for LimineRequest<Resp, Req> {}

impl<Resp, Req> LimineRequest<Resp, Req> {
    pub const fn new(id: [u64; 2], revision: u64, request: Req) -> Self {
        Self {
            common_magic: COMMON_MAGIC,
            id,
            revision,
            response: UnsafeCell::new(core::ptr::null_mut::<LimineResponse<Resp>>()),
            request,
        }
    }

    pub fn get_response(&self) -> Option<&LimineResponse<Resp>> {
        let ptr: *mut LimineResponse<Resp> = unsafe { *self.response.get() };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { &*ptr })
        }
    }
}
