use syn::parse_quote;
use syn::spanned::Spanned;
use syn::Field;
use syn::Fields;
use syn::Ident;
use syn::Lit;
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
    let field_span = field.span();

    if let Some(ident) = &field.ident {
        let lit_str = LitStr::new(&ident.to_string(), field_span);
        let field_key = Lit::Str(lit_str);

        parse_quote! {
            let #ident = attribution::FromParameters::from_parameters(&mut attr_args, &#field_key.into())?;
        }
    } else {
        let ident_name = format!("_{}", position);
        let ident = Ident::new(&ident_name, field_span);

        let lit_int = LitInt::new(&format!("{}usize", position), field_span);
        let field_key = Lit::Int(lit_int);
        parse_quote! {
            let #ident = attribution::FromParameters::from_parameters(&mut attr_args, &#field_key.into())?;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use proc_macro2::Span;
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

        assert_eq!(actual, expected);
    }

    #[test]
    fn build_unnamed_field_extractor_test() {
        let raw_field = build_test_field(false);

        let actual = build_extractor(0, &raw_field);
        let expected: Stmt = parse_quote! {
            let _0 = attribution::FromParameters::from_parameters(&mut attr_args, &0usize.into()).unwrap();
        };

        assert_eq!(actual, expected);
    }
}
