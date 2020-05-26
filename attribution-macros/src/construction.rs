use crate::identifiers::build_unnamed_idents;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;
use syn::punctuated::Punctuated;
use syn::Expr;
use syn::Fields;
use syn::FieldsNamed;
use syn::FieldsUnnamed;
use syn::Ident;
use syn::ItemStruct;
use syn::Token;
use syn::Variant;

pub fn build_struct_constructor(input_struct: &ItemStruct) -> Expr {
    let struct_name = &input_struct.ident;

    let ctor_body = match &input_struct.fields {
        Fields::Named(named) => named_constructor_body(named),
        Fields::Unnamed(unnamed) => unnamed_constructor_body(unnamed),
        Fields::Unit => quote! { {} },
    };

    parse_quote! {
        Ok(#struct_name #ctor_body)
    }
}

pub fn build_variant_constructor(enum_name: &Ident, variant: &Variant) -> Expr {
    let variant_name = &variant.ident;

    let constructor_body = match &variant.fields {
        Fields::Named(named) => named_constructor_body(named),
        Fields::Unnamed(unnamed) => unnamed_constructor_body(unnamed),
        Fields::Unit => quote! { {} },
    };

    parse_quote! {
        Ok(#enum_name::#variant_name #constructor_body)
    }
}

/// Builds the constructor body for a struct with named fields.
fn named_constructor_body(FieldsNamed { named, .. }: &FieldsNamed) -> TokenStream {
    let idents: Punctuated<&Ident, Token![,]> =
        named.iter().map(|el| el.ident.as_ref().unwrap()).collect();
    parse_quote! { { #idents } }
}

/// Builds the constructor body for a struct with unnamed fields.
fn unnamed_constructor_body(FieldsUnnamed { unnamed, .. }: &FieldsUnnamed) -> TokenStream {
    let idents: Punctuated<Ident, Token![,]> = build_unnamed_idents(unnamed.len()).collect();
    quote! { (#idents) }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn build_struct_constructor_test() {
        let input_struct = parse_quote! {
            struct Foo {
                a: u32,
                b: String,
                c: bool
            }
        };

        let actual = build_struct_constructor(&input_struct);
        let expected: Expr = parse_quote! {
            Ok(Foo { a, b, c })
        };

        assert_eq!(
            expected, actual,
            "\nExpected: {:#?}\nActual: {:#?}",
            expected, actual
        )
    }

    #[test]
    fn build_variant_constructor_test() {
        let enum_ident: Ident = parse_quote! { EnumName };
        let variant = parse_quote! {
            Foo (u32, String, bool)
        };

        let actual = build_variant_constructor(&enum_ident, &variant);
        let expected: Expr = parse_quote! {
            Ok(EnumName::Foo(_0, _1, _2))
        };

        assert_eq!(
            expected, actual,
            "\nExpected: {:#?}\nActual: {:#?}",
            expected, actual
        )
    }

    #[test]
    fn named_constructor_body_test() {
        let fields_named = parse_quote! {
            { a: u32, b: String, c: bool }
        };

        let actual = named_constructor_body(&fields_named);
        let expected = quote! {
            { a, b, c }
        };

        assert_eq!(expected.to_string(), actual.to_string())
    }

    #[test]
    fn unnamed_constructor_body_test() {
        let fields_unnamed = parse_quote! {
            (u32, String, bool)
        };

        let actual = unnamed_constructor_body(&fields_unnamed);
        let expected = quote! {
            (_0, _1, _2)
        };

        assert_eq!(expected.to_string(), actual.to_string());
    }
}
