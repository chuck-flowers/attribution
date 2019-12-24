#![warn(clippy::all, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
extern crate proc_macro;

mod extraction;
mod field_spec;

use field_spec::FieldSpec;
use proc_macro2::TokenStream;
use quote::quote;
use std::convert::TryFrom;
use syn::parse_macro_input;
use syn::Fields;
use syn::Ident;
use syn::Item;
use syn::ItemStruct;

/// The attribute that is used to generate the parsing logic for a struct
/// representing the parameters for an attribute.
#[proc_macro_attribute]
pub fn attr_args(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Parse the inputs
    let input_struct = if let Item::Struct(struct_data) = parse_macro_input!(input as Item) {
        struct_data
    } else {
        panic!("The attribute can only be applied to structs")
    };

    let fields = extract_fields(&input_struct);
    let output = impl_parse(&input_struct.ident, &fields);

    (quote! {
        #input_struct
        #output
    })
    .into()
}

fn extract_fields(input_struct: &ItemStruct) -> Vec<FieldSpec> {
    if let Fields::Named(fields) = &input_struct.fields {
        fields
            .named
            .iter()
            .map(TryFrom::try_from)
            .map(Result::unwrap)
            .collect()
    } else {
        panic!("The attribute can only be applied to structs with named fields")
    }
}

fn impl_parse(struct_name: &Ident, fields: &[FieldSpec]) -> TokenStream {
    let extractors = fields.iter().map(extraction::build_extractor);

    let extraction = quote! {
        #(#extractors)*
    };

    let idents = fields.iter().map(FieldSpec::ident);
    let struct_return = quote! {
        Ok(#struct_name {
            #(#idents),*
        })
    };

    quote! {
        impl syn::parse::Parse for #struct_name {
            fn parse(buffer: &syn::parse::ParseBuffer) -> syn::parse::Result<Self> {
                use std::convert::TryInto;
                let mut attr_args = <attribution::Parameters as syn::parse::Parse>::parse(buffer)?;
                #extraction
                #struct_return
            }
        }
    }
}
