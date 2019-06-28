extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::AttributeArgs;
use syn::Item;

#[proc_macro_attribute]
pub fn attr_args(attr: TokenStream, input: TokenStream) -> TokenStream {
    unimplemented!()
}