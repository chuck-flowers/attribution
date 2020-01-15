use proc_macro2::Span as Span2;
use syn::parse_quote;
use syn::Field;
use syn::Fields;
use syn::Ident;
use syn::LitInt;
use syn::LitStr;
use syn::Stmt;

pub fn build_extractors<'a>(fields: &'a Fields) -> impl Iterator<Item = Stmt> + 'a {
    fields
        .iter()
        .enumerate()
        .map(|(i, field)| build_extractor(i, field))
}

fn build_extractor(position: usize, field: &Field) -> Stmt {
    if let Some(ident) = &field.ident {
        let field_key = LitStr::new(&ident.to_string(), Span2::call_site());
        parse_quote! {
            let #ident = attribution::FromParameters::from_parameters(&mut attr_args, &#field_key.into()).unwrap();
        }
    } else {
        let ident_name = format!("_{}", position);
        let ident = Ident::new(&ident_name, Span2::call_site());

        let unnamed_key = format!("{}usize", position);
        let field_key = LitInt::new(&unnamed_key, Span2::call_site());
        parse_quote! {
            let #ident = attribution::FromParameters::from_parameters(&mut attr_args, &#field_key.into()).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use proc_macro2::Span;
    use quote::ToTokens;
    use syn::parse_quote;
    use syn::Field;

    fn build_test_field(use_name: bool) -> Field {
        let attrs = vec![];
        let vis = syn::Visibility::Inherited;
        let ident = if use_name {
            Some(parse_quote!(foo))
        } else {
            None
        };
        let colon_token = Some(syn::token::Colon {
            spans: [Span::call_site()],
        });
        let ty = parse_quote!(bool);

        Field {
            attrs,
            vis,
            ident,
            colon_token,
            ty,
        }
    }

    #[test]
    fn build_named_field_extractor_test() {
        let raw_field = build_test_field(true);

        let actual = build_extractor(0, &raw_field);
        let expected: Stmt = parse_quote! {
            let foo = attribution::FromParameters::from_parameters(&mut attr_args, &"foo".into()).unwrap();
        };

        assert_eq!(
            actual.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn build_unnamed_field_extractor_test() {
        let raw_field = build_test_field(false);

        let actual = build_extractor(0, &raw_field);
        let expected: Stmt = parse_quote! {
            let _0 = attribution::FromParameters::from_parameters(&mut attr_args, &0usize.into()).unwrap();
        };

        assert_eq!(
            actual.to_token_stream().to_string(),
            expected.to_token_stream().to_string()
        );
    }
}
