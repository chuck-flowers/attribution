use proc_macro2::Span as Span2;
use syn::Ident;
use syn::ItemEnum;
use syn::Variant;

/// Builds an iterator that generates a specified number of unnamed idents
pub fn build_unnamed_idents(num: usize) -> impl Iterator<Item = Ident> {
    (0..num).map(|i| Ident::new(&format!("_{}", i), Span2::call_site()))
}

pub fn build_variant_parser_idents<'a>(
    input_enum: &'a ItemEnum,
) -> impl Iterator<Item = Ident> + 'a {
    input_enum.variants.iter().map(build_variant_parser_ident)
}

pub fn build_variant_parser_ident(variant: &Variant) -> Ident {
    let ident_value = format!("parse_{}", variant.ident);
    Ident::new(&ident_value, variant.ident.span())
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use syn::parse_quote;

    #[test]
    fn build_unnamed_idents_test() {
        let _0: Ident = parse_quote! { _0 };
        let _1: Ident = parse_quote! { _1 };
        let _2: Ident = parse_quote! { _2 };

        let mut actual = build_unnamed_idents(3);

        assert_eq!(Some(_0), actual.next());
        assert_eq!(Some(_1), actual.next());
        assert_eq!(Some(_2), actual.next());
        assert_eq!(None, actual.next());
    }

    #[test]
    fn build_variant_parser_ident_test() {
        let variant = parse_quote! {
            Foo { a: u32, b: String, c: bool }
        };

        let actual = build_variant_parser_ident(&variant);
        let expected: Ident = parse_quote! {
            parse_Foo
        };

        assert_eq!(expected, actual);
    }
}
