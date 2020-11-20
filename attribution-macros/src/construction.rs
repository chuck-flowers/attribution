use crate::identifiers::build_unnamed_idents;
use syn::parse_quote;
use syn::Expr;
use syn::Fields;
use syn::Ident;
use syn::ItemStruct;
use syn::Variant;

pub fn build_struct_constructor(input_struct: &ItemStruct) -> Expr {
    let struct_name = &input_struct.ident;

    match &input_struct.fields {
        Fields::Named(named) => {
            let idents = named
                .named
                .iter()
                .map(|field| field.ident.as_ref().unwrap());
            parse_quote! { Ok(#struct_name { #(#idents),* }) }
        }
        Fields::Unnamed(unnamed) => {
            let idents = build_unnamed_idents(unnamed.unnamed.len());
            parse_quote! { Ok(#struct_name (#(#idents),*)) }
        }
        Fields::Unit => parse_quote! { Ok(#struct_name {}) },
    }
}

pub fn build_variant_constructor(enum_name: &Ident, variant: &Variant) -> Expr {
    let variant_name = &variant.ident;

    match &variant.fields {
        Fields::Named(named) => {
            let idents = named
                .named
                .iter()
                .map(|field| field.ident.as_ref().unwrap());
            parse_quote! {Ok(#enum_name::#variant_name {#(#idents),*})}
        }
        Fields::Unnamed(unnamed) => {
            let idents = build_unnamed_idents(unnamed.unnamed.len());
            parse_quote! { Ok(#enum_name::#variant_name ( #(#idents),* )) }
        }
        Fields::Unit => parse_quote! { Ok(#enum_name::#variant_name {}) },
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

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

        assert_eq!(expected, actual)
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

        assert_eq!(expected, actual)
    }
}
