#![warn(clippy::all)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::AttributeArgs;
use syn::Item;

#[proc_macro_attribute]
pub fn attr_args(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_attr = parse_macro_input!(attr as AttributeArgs);
    let input_struct = parse_macro_input!(input as Item);

    (quote! {
        #input_struct
    })
    .into()
}