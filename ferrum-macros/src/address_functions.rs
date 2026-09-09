use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn address_functions(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded: TokenStream2 = quote! {
        impl #name {
            pub const fn new(address: usize) -> Self {
                Self(address)
            }

            pub const fn as_usize(&self) -> usize {
                self.0
            }

            pub const fn as_non_null<T>(&self) -> core::ptr::NonNull<T> {
                core::ptr::NonNull::new(core::ptr::with_exposed_provenance_mut::<T>(self.0)).expect("Tried to create NonNull from address, found null")
            }

            pub const fn is_aligned(&self, alignment: core::ptr::Alignment) -> bool {
                self.0 & (alignment.as_usize() - 1) == 0
            }

            pub const fn align_up(mut self, alignment: core::ptr::Alignment) -> Self {
                self.0 = (self.0 + (alignment.as_usize() - 1)) & !(alignment.as_usize() - 1);
                self
            }

            pub const fn align_down(mut self, alignment: core::ptr::Alignment) -> Self {
                self.0 &= !(alignment.as_usize() - 1);
                self
            }

            pub const fn alignment(&self) -> core::ptr::Alignment {
                unsafe {
                    if self.0 == 0 {
                        core::ptr::Alignment::new_unchecked(1 << (usize::BITS - 1))
                    } else {
                        core::ptr::Alignment::new_unchecked(1 << self.0.trailing_zeros())
                    }
                }
            }
        }
    };

    expanded.into()
}
