#[macro_export]
macro_rules! csr {
    ($(#[$doc:meta])*
     $ty:ident,
     $mask:expr,
     $csr_number:literal) => {
        #[repr(C)]
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $ty {
            bits: usize,
        }

        impl $ty {
            pub const BITMASK: usize = $mask;

            pub const fn from_bits(bits: usize) -> Self {
                Self { bits: bits & $mask }
            }

            pub const fn bits(&self) -> usize {
                self.bits & $mask
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
        }
    };
}
