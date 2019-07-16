use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use crate::field_spec::FieldSpec;

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

    #[test]
    fn build_extractor_test() {
        let ident = syn::parse_quote!(foo);
        let ty = syn::parse_quote!(bool);
        let field = FieldSpec::new(&ident, &ty);
        
        let extractor = build_extractor(&field);
        let expected = quote! {
            let foo = attr_args.remove("foo").unwrap().try_into().unwrap();
        };

        assert_eq!(extractor.to_string(), expected.to_string());
    }
}