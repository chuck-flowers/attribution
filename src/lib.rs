#![warn(clippy::all)]

extern crate proc_macro;

mod field_spec;

use std::convert::TryFrom;
use field_spec::FieldSpec;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn attr_args(attr: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the inputs
    let input_attr = syn::parse_macro_input!(attr as syn::AttributeArgs);
    let input_struct = if let syn::Item::Struct(struct_data) = syn::parse_macro_input!(input as syn::Item) {
        struct_data
    } else {
        panic!("The attribute can only be applied to structs")
    };

    let fields = extract_fields(&input_struct);

    (quote! {
        #input_struct
    })
    .into()
}

fn extract_fields(input_struct: &syn::ItemStruct) -> Vec<FieldSpec> {
    if let syn::Fields::Named(fields) = &input_struct.fields {
        fields.named.iter().map(TryFrom::try_from).map(Result::unwrap).collect()
    } else {
        panic!("The attribute can only be applied to structs with named fields")
    }
}