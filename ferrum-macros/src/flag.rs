use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    Ident, LitInt, Token,
};

struct FlagInput {
    name: Ident,
    bit: LitInt,
}

impl Parse for FlagInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let bit: LitInt = input.parse()?;
        Ok(FlagInput { name, bit })
    }
}

pub fn flag(input: TokenStream) -> TokenStream {
    let FlagInput { name, bit } = syn::parse_macro_input!(input as FlagInput);

    let set_ident = format_ident!("set_{}", name);
    let unset_ident = format_ident!("unset_{}", name);
    let is_ident = format_ident!("is_{}", name);

    let expanded: TokenStream2 = quote! {
        #[inline]
        #[allow(dead_code)]
        pub fn #set_ident(&mut self) {
            unsafe {
                let ptr: *mut usize = (self as *mut Self).cast::<usize>();
                let val: usize = core::ptr::read_volatile(ptr);
                core::ptr::write_volatile(ptr, val | (1 << #bit) as usize)
            }
        }

        #[inline]
        #[allow(dead_code)]
        pub const fn #name(self) -> Self {
            Self(self.0 | (1 << #bit))
        }

        #[inline]
        #[allow(dead_code)]
        pub fn #unset_ident(&mut self) {
            unsafe {
                let ptr: *mut usize = (self as *mut Self).cast::<usize>();
                let val: usize = core::ptr::read_volatile(ptr);
                core::ptr::write_volatile(ptr, val & !(1 << #bit) as usize)
            }
        }

        #[inline]
        #[allow(dead_code)]
        pub fn #is_ident(&self) -> bool {
            unsafe {
                core::ptr::read_volatile((self as *const Self).cast::<usize>()) & ((1 << #bit) as usize) != 0
            }
        }
    };

    expanded.into()
}
