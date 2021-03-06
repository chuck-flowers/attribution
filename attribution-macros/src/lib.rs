#![warn(clippy::all)]

extern crate proc_macro;

mod construction;
mod extraction;
mod identifiers;

use self::construction::build_struct_constructor;
use self::construction::build_variant_constructor;
use self::extraction::build_extractors;
use self::identifiers::build_variant_parser_ident;
use self::identifiers::build_variant_parser_idents;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::parse_quote;
use syn::Ident;
use syn::Item;
use syn::ItemEnum;
use syn::ItemFn;
use syn::ItemImpl;
use syn::ItemStruct;
use syn::Variant;

/// The derive macro that is used to generate the parsing logic for a struct
/// representing the parameters for an attribute.
#[proc_macro_derive(AttrArgs)]
pub fn attr_args(input: TokenStream) -> TokenStream {
    let output = match parse_macro_input!(input as Item) {
        Item::Struct(input_struct) => {
            let impl_item = impl_parse_for_struct(&input_struct);
            quote! { #impl_item }
        }
        Item::Enum(input_enum) => {
            let impl_item = impl_parse_for_enum(&input_enum);
            quote! { #impl_item }
        }
        _ => panic!("The attribute can only be applied to structs and enums"),
    };

    output.into()
}

/// Creates the impl body for a tagged struct
fn impl_parse_for_struct(input_struct: &ItemStruct) -> ItemImpl {
    let struct_name = &input_struct.ident;

    // Build the statements that pull out the field values from the Parameters
    let field_extractors = build_extractors(&input_struct.fields);

    // Build the statement that constructs the struct
    let struct_return = build_struct_constructor(&input_struct);

    parse_quote! {
        impl syn::parse::Parse for #struct_name {
            fn parse(buffer: &syn::parse::ParseBuffer) -> syn::parse::Result<Self> {
                let mut attr_args = <attribution::Parameters as syn::parse::Parse>::parse(buffer)?;

                #(#field_extractors)*

                #struct_return
            }
        }
    }
}

/// Creates the impl body for a tagged enum
fn impl_parse_for_enum(input_enum: &ItemEnum) -> ItemImpl {
    let enum_name = &input_enum.ident;
    let parser_idents = build_variant_parser_idents(&input_enum);

    // Builds a function used to try to parse each variant of the enum
    let parser_decls = input_enum
        .variants
        .iter()
        .map(|variant| build_variant_parser(&input_enum.ident, variant));

    parse_quote! {

        impl syn::parse::Parse for #enum_name {
            fn parse(buffer: &syn::parse::ParseBuffer) -> syn::parse::Result<Self> {
                #(#parser_decls)*

                #(
                    if let Ok(val) = buffer.call(#parser_idents) {
                        return Ok(val);
                    }
                )*

                else {
                    return Err(buffer.error("No matching variant found."));
                }
            }
        }
    }
}

/// Constructs a function that will attempt to parse a token stream as a
/// provided enum variant.
fn build_variant_parser(enum_name: &Ident, variant: &Variant) -> ItemFn {
    let parser_ident = build_variant_parser_ident(variant);

    let extractors = build_extractors(&variant.fields);
    let constructor = build_variant_constructor(enum_name, variant);

    parse_quote! {
        #[allow(non_snake_case)]
        fn #parser_ident(buffer: &syn::parse::ParseBuffer) -> syn::parse::Result<#enum_name> {
            let mut attr_args = buffer.parse::<attribution::Parameters>()?;

            #(#extractors)*

            #constructor
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use syn::parse_quote;

    #[test]
    fn impl_parse_for_struct_unnamed_struct_test() {
        let input_struct: ItemStruct = parse_quote! {
            struct Foo(u64, u64);
        };

        let actual = impl_parse_for_struct(&input_struct);
        let expected: ItemImpl = parse_quote! {
            impl syn::parse::Parse for Foo {
                fn parse(buffer: &syn::parse::ParseBuffer) -> syn::parse::Result<Self> {
                    let mut attr_args = <attribution::Parameters as syn::parse::Parse>::parse(buffer)?;

                    let _0 = attribution::FromParameters::from_parameters(&mut attr_args, &0usize.into())?;
                    let _1 = attribution::FromParameters::from_parameters(&mut attr_args, &1usize.into())?;

                    Ok(Foo(_0, _1))
                }
            }
        };

        assert_eq!(expected, actual);
    }
}
