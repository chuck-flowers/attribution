#![warn(clippy::all)]

extern crate proc_macro;

mod extraction;
mod field_spec;

use std::convert::TryFrom;
use field_spec::FieldSpec;
use attribution_types::AttrMap;
use proc_macro2::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn attr_args(attr: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the inputs
    let input_attr = syn::parse_macro_input!(attr as AttrMap);
    let input_struct = if let syn::Item::Struct(struct_data) = syn::parse_macro_input!(input as syn::Item) {
        struct_data
    } else {
        panic!("The attribute can only be applied to structs")
    };

    let fields = extract_fields(&input_struct);
    let output = impl_parse(&input_attr, &input_struct.ident, &fields);

    (quote! {
        #input_struct
        #output
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

fn impl_parse(_input_attr: &AttrMap, struct_name: &syn::Ident, fields: &Vec<FieldSpec>) -> TokenStream {
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
                let mut attr_args = attribution::AttrMap::parse(buffer)?;
                #extraction
                #struct_return
            }
        }
    }
}