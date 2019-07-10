#![warn(clippy::all)]

extern crate proc_macro;

mod field_spec;
mod attr_map;

use std::convert::TryFrom;
use field_spec::FieldSpec;
use attr_map::AttrMap;
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

fn impl_parse(input_attr: &AttrMap, struct_name: &syn::Ident, fields: &Vec<FieldSpec>) -> TokenStream {
    let idents = fields.iter().map(FieldSpec::ident);
    let tys = fields.iter().map(FieldSpec::ty);

    let inital_field_declarations = generate_field_decl(idents.clone(), tys.clone());
    let struct_return = generate_struct_return(&struct_name, idents.clone());

    quote! {
        // Declare each field for the struct
        #inital_field_declarations

        // Return the struct
        #struct_return
    }
}

fn generate_field_decl<'a, 'b>(idents: impl Iterator<Item = &'a syn::Ident>, tys: impl Iterator<Item = &'b syn::Type>) -> TokenStream {
    quote! {
        #(let mut #idents : Option<#tys> = None;)*
    }
}

fn generate_struct_return<'a, 'b>(struct_name: &syn::Ident, idents: impl Iterator<Item = &'a syn::Ident> + Clone) -> TokenStream {
    let struct_return_idents_1 = idents.clone();
    let struct_return_idents_2 = idents.clone();
    quote! {
        Ok( #struct_name {
            #(#struct_return_idents_1: #struct_return_idents_2.unwrap()),*
        })
    }
}