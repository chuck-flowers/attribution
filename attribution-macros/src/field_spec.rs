use proc_macro2::TokenStream;
use quote::ToTokens;
use std::convert::TryFrom;
use syn::Field;

pub struct FieldSpec<'i, 't> {
    ident: &'i syn::Ident,
    ty: &'t syn::Type,
}

impl<'i, 't> FieldSpec<'i, 't> {
    pub fn new(ident: &'i syn::Ident, ty: &'t syn::Type) -> Self {
        FieldSpec { ident, ty }
    }

    pub fn ident(&self) -> &syn::Ident {
        self.ident
    }

    pub fn ty(&self) -> &syn::Type {
        self.ty
    }
}

impl std::fmt::Debug for FieldSpec<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ty_stream = TokenStream::new();
        self.ty().to_tokens(&mut ty_stream);

        let mut ident_stream = TokenStream::new();
        self.ty().to_tokens(&mut ident_stream);

        write!(
            f,
            "FieldSpec {{ ident: {:?}, ty: {:?} }}",
            ident_stream, ty_stream
        )
    }
}

impl std::cmp::PartialEq for FieldSpec<'_, '_> {
    fn eq(&self, other: &Self) -> bool {
        let mut self_ty_stream = TokenStream::new();
        self.ty().to_tokens(&mut self_ty_stream);

        let mut other_ty_stream = TokenStream::new();
        other.ty().to_tokens(&mut other_ty_stream);

        self.ident == other.ident && self_ty_stream.to_string() == other_ty_stream.to_string()
    }
}

#[derive(Debug)]
pub enum ParseError {
    NoIdent,
}

impl<'i, 't, 'a: 'i + 't> TryFrom<&'a Field> for FieldSpec<'i, 't> {
    type Error = ParseError;

    fn try_from(field: &'a Field) -> Result<Self, Self::Error> {
        let field_ident = &field.ident;
        if let Some(ident) = field_ident {
            let ty = &field.ty;
            Ok(FieldSpec { ident, ty })
        } else {
            Err(ParseError::NoIdent)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use syn::parse_quote;

    use syn::Fields;
    use syn::ItemStruct;
    #[test]
    fn try_from_test() {
        let test_struct: ItemStruct = parse_quote! {
            struct TestStruct {
                foo: i32,
                bar: String
            }
        };

        if let Fields::Named(named_fields) = test_struct.fields {
            let test_fields: Vec<FieldSpec> = named_fields
                .named
                .iter()
                .map(|f| FieldSpec::try_from(f).unwrap())
                .collect();

            let foo_ident: syn::Ident = parse_quote!(foo);
            let foo_type: syn::Type = parse_quote!(i32);

            let bar_ident: syn::Ident = parse_quote!(bar);
            let bar_type: syn::Type = parse_quote!(String);

            assert_eq!(
                test_fields,
                vec![
                    FieldSpec {
                        ident: &foo_ident,
                        ty: &foo_type
                    },
                    FieldSpec {
                        ident: &bar_ident,
                        ty: &bar_type
                    }
                ]
            )
        }
    }
}
