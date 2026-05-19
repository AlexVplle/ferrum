mod address_functions;
mod flag;

use proc_macro::TokenStream;

#[proc_macro_derive(AddressFunctions)]
pub fn address_functions(input: TokenStream) -> TokenStream {
    address_functions::address_functions(input)
}

#[proc_macro]
pub fn flag(input: TokenStream) -> TokenStream {
    flag::flag(input)
}
