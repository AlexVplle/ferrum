use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn address_functions(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded: TokenStream2 = quote! {
        impl core::fmt::Display for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{:#x}", self.0)
            }
        }

        impl core::fmt::Debug for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({:#x})", stringify!(#name), self.0)
            }
        }

        impl core::ops::Add<usize> for #name {
            type Output = Self;
            fn add(self, rhs: usize) -> Self {
                Self(self.0.wrapping_add(rhs))
            }
        }

        impl core::ops::Sub<usize> for #name {
            type Output = Self;
            fn sub(self, rhs: usize) -> Self {
                Self(self.0.wrapping_sub(rhs))
            }
        }

        impl core::ops::Sub<#name> for #name {
            type Output = usize;
            fn sub(self, rhs: #name) -> usize {
                self.0.wrapping_sub(rhs.0)
            }
        }

        impl core::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl core::cmp::PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl core::cmp::PartialEq<usize> for #name {
            fn eq(&self, other: &usize) -> bool {
                self.0 == *other
            }
        }

        impl core::cmp::PartialOrd<usize> for #name {
            fn partial_cmp(&self, other: &usize) -> Option<core::cmp::Ordering> {
                self.0.partial_cmp(other)
            }
        }

        impl core::ops::AddAssign<usize> for #name {
            fn add_assign(&mut self, rhs: usize) {
                self.0 = self.0.wrapping_add(rhs);
            }
        }

        impl core::ops::SubAssign<usize> for #name {
            fn sub_assign(&mut self, rhs: usize) {
                self.0 = self.0.wrapping_sub(rhs);
            }
        }

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
