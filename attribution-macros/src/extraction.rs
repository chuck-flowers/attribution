use crate::field_spec::FieldSpec;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

pub fn build_extractor(field: &FieldSpec) -> TokenStream {
    let var_ident = field.ident().clone();
    let field_key = syn::LitStr::new(&field.ident().to_string(), Span::call_site());

    quote! {
        let #var_ident = attr_args.remove(#field_key)
            .unwrap()
            .try_into()
            .unwrap();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::field_spec::FieldSpec;
    use proc_macro2::Span;
    use std::convert::TryFrom;
    use syn::parse_quote;

    #[test]
    fn build_extractor_test() {
        let raw_field = syn::Field {
            attrs: vec![],
            vis: syn::Visibility::Inherited,
            ident: Some(parse_quote!(foo)),
            colon_token: Some(syn::token::Colon {
                spans: [Span::call_site()],
            }),
            ty: parse_quote!(bool),
        };
        let field = FieldSpec::try_from(&raw_field).unwrap();

        let extractor = build_extractor(&field);
        let expected = quote! {
            let foo = attr_args.remove("foo").unwrap().try_into().unwrap();
        };

        assert_eq!(extractor.to_string(), expected.to_string());
    }
}
