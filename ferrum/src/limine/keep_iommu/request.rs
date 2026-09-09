use crate::limine::request::LimineRequest;

#[used]
#[unsafe(link_section = ".limine_requests")]
pub static KEEP_IOMMU_REQUEST: LimineRequest<()> =
    LimineRequest::new([0x8ebaabe51f490179, 0x2aa86a59ffb4ab0f], 0, ());
