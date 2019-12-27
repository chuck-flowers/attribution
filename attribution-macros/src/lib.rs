#![warn(clippy::all, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
extern crate proc_macro;

mod extraction;
mod field_spec;

use field_spec::FieldSpec;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::convert::TryFrom;
use syn::parse_macro_input;
use syn::Fields;
use syn::Item;
use syn::ItemEnum;
use syn::ItemStruct;

/// The attribute that is used to generate the parsing logic for a struct
/// representing the parameters for an attribute.
#[proc_macro_attribute]
pub fn attr_args(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let output = match parse_macro_input!(input as Item) {
        Item::Struct(input_struct) => impl_parse_for_struct(&input_struct),
        Item::Enum(input_enum) => impl_parse_for_enum(&input_enum),
        _ => panic!("The attribute can only be applied to structs"),
    };

    output.into()
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

fn impl_parse_for_struct(input_struct: &ItemStruct) -> TokenStream2 {
    let fields = extract_fields(&input_struct);
    let extractors = fields.iter().map(extraction::build_extractor);

    let extraction = quote! {
        #(#extractors)*
    };

    let idents = fields.iter().map(FieldSpec::ident);
    let struct_name = &input_struct.ident;
    let struct_return = quote! {
        Ok(#struct_name {
            #(#idents),*
        })
    };

    quote! {
        #input_struct

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

fn impl_parse_for_enum(input_enum: &ItemEnum) -> TokenStream2 {
    todo!()
}
