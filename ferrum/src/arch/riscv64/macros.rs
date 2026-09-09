#[macro_export]
macro_rules! csr {
    ($(#[$doc:meta])*
     $ty:ident,
     $mask:expr,
     $csr_number:literal) => {
        #[repr(C)]
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $ty(usize);

        impl $ty {
            pub const BITMASK: usize = $mask;

            pub const fn from_bits(bits: usize) -> Self {
                Self(bits & $mask)
            }

            pub const fn bits(&self) -> usize {
                self.0 & $mask
            }

            pub const fn bitmask(&self) -> usize {
                Self::BITMASK
            }

            pub fn read() -> Self {
                let bits: usize;
                unsafe {
                    core::arch::asm!(
                        concat!("csrr {0}, ", stringify!($csr_number)),
                        out(reg) bits,
                    );
                }
                Self::from_bits(bits)
            }

            pub fn write(value: Self) {
                unsafe {
                    core::arch::asm!(
                        concat!("csrw ", stringify!($csr_number), ", {0}"),
                        in(reg) value.0,
                    );
                }
            }

            pub fn set_bits(mask: usize) {
                unsafe {
                    core::arch::asm!(
                        concat!("csrs ", stringify!($csr_number), ", {0}"),
                        in(reg) mask,
                    );
                }
            }

            pub fn clear_bits(mask: usize) {
                unsafe {
                    core::arch::asm!(
                        concat!("csrc ", stringify!($csr_number), ", {0}"),
                        in(reg) mask,
                    );
                }
            }
        }
    };
}
