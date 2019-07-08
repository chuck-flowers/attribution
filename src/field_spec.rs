use proc_macro2::TokenStream;
use quote::ToTokens;
use std::convert::TryFrom;
use syn::Field;

#[derive(Debug, Eq, PartialEq)]
pub struct FieldSpec {
    ident: String,
    ty: String,
}

#[derive(Debug)]
pub enum ParseError {
    NoIdent,
}

impl TryFrom<&Field> for FieldSpec {
    type Error = ParseError;

    fn try_from(field: &Field) -> Result<Self, Self::Error> {
        let field_ident = &field.ident;
        if let Some(ident_ref) = field_ident {
            let ident = ident_ref.to_string();

            let mut ty_stream = TokenStream::new();
            field.ty.to_tokens(&mut ty_stream);
            let ty = format!("{}", ty_stream);

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
            assert_eq!(
                test_fields,
                vec![
                    FieldSpec {
                        ident: "foo".to_string(),
                        ty: "i32".to_string()
                    },
                    FieldSpec {
                        ident: "bar".to_string(),
                        ty: "String".to_string()
                    }
                ]
            )
        }
    }
}
