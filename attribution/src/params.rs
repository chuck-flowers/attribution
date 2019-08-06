use crate::conversion::FromParameters;
use crate::conversion::FromParametersError;
use std::collections::HashMap;
use std::convert::TryInto;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::Lit;
use syn::Meta;
use syn::NestedMeta;

pub struct Parameters(HashMap<String, ParamVal>);

impl Parameters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<&ParamVal> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: String, val: ParamVal) -> Option<ParamVal> {
        self.0.insert(key, val)
    }

    pub fn remove(&mut self, key: &str) -> Option<ParamVal> {
        self.0.remove(key)
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters(HashMap::new())
    }
}

impl Parse for Parameters {
    fn parse(buffer: ParseStream) -> syn::parse::Result<Self> {
        let mut attribute_map = Self::new();

        while !buffer.is_empty() {
            // Parse the next key value pair
            if let NestedMeta::Meta(Meta::NameValue(nv)) = buffer.parse()? {
                let param_name = nv.ident.to_string();
                let param_value = ParamVal::from(&nv.lit);
                attribute_map.insert(param_name, param_value);
            }

            // If the next token is a comma, consume it
            if buffer.peek(syn::Token![,]) {
                let _ = buffer.parse::<syn::token::Comma>();
            }
        }

        Ok(attribute_map)
    }
}

pub struct DynamicParameters(Parameters);

impl FromParameters for DynamicParameters {
    fn from_parameters(
        params: &mut Parameters,
        _param_name: &str,
    ) -> Result<Self, FromParametersError> {
        let mut ret = DynamicParameters(Parameters::new());
        let keys: Vec<String> = params.0.keys().map(|s| s.into()).collect();
        for key in keys {
            let val = params.remove(&key).unwrap();
            ret.0.insert(key, val);
        }

        Ok(ret)
    }
}

#[cfg(test)]
mod attr_map_tests {
    use super::*;
    use syn::parse2;
    use syn::parse_quote;
    use syn::Attribute;

    #[test]
    fn parse_test() {
        let attr: Attribute = parse_quote!(#[attr(foo = "fooValue", bar = 1, baz = true)]);
        if let proc_macro2::TokenTree::Group(group) = attr.tts.into_iter().next().unwrap() {
            let attr_args: Parameters = parse2(group.stream()).unwrap();
            let foo_val = attr_args.get("foo");
            let bar_val = attr_args.get("bar");
            let baz_val = attr_args.get("baz");
            let other_val = attr_args.get("other");

            assert_eq!(foo_val, Some(&ParamVal::Str("fooValue".to_string())));
            assert_eq!(bar_val, Some(&ParamVal::Int(1)));
            assert_eq!(baz_val, Some(&ParamVal::Bool(true)));
            assert_eq!(other_val, None);
        } else {
            panic!("Didn't unwrap appropriately");
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParamVal {
    Bool(bool),
    Int(u64),
    Str(String),
}

impl From<&Lit> for ParamVal {
    fn from(lit: &Lit) -> Self {
        match lit {
            Lit::Bool(b) => ParamVal::Bool(b.value),
            Lit::Int(i) => ParamVal::Int(i.value()),
            Lit::Str(s) => ParamVal::Str(s.value()),
            _ => unimplemented!(),
        }
    }
}

impl From<Lit> for ParamVal {
    fn from(lit: Lit) -> Self {
        ParamVal::from(&lit)
    }
}

#[derive(Debug, PartialEq)]
pub struct TryIntoParamValError;

impl TryInto<bool> for ParamVal {
    type Error = TryIntoParamValError;
    fn try_into(self) -> Result<bool, Self::Error> {
        if let ParamVal::Bool(b) = self {
            Ok(b)
        } else {
            Err(TryIntoParamValError {})
        }
    }
}

impl TryInto<u64> for ParamVal {
    type Error = TryIntoParamValError;
    fn try_into(self) -> Result<u64, Self::Error> {
        if let ParamVal::Int(i) = self {
            Ok(i)
        } else {
            Err(TryIntoParamValError {})
        }
    }
}

impl TryInto<String> for ParamVal {
    type Error = TryIntoParamValError;
    fn try_into(self) -> Result<String, Self::Error> {
        if let ParamVal::Str(s) = self {
            Ok(s)
        } else {
            Err(TryIntoParamValError {})
        }
    }
}

impl From<&str> for ParamVal {
    fn from(src: &str) -> Self {
        ParamVal::Str(src.into())
    }
}

impl From<u64> for ParamVal {
    fn from(src: u64) -> Self {
        ParamVal::Int(src)
    }
}

impl From<bool> for ParamVal {
    fn from(src: bool) -> Self {
        ParamVal::Bool(src)
    }
}

#[cfg(test)]
mod attr_val_tests {

    use super::*;
    use proc_macro2::Span;
    use syn::LitStr;

    #[test]
    fn from_test() {
        let cases = vec![
            (
                Lit::Str(LitStr::new("foo", Span::call_site())),
                ParamVal::Str("foo".to_string()),
                "string literal",
            ),
            (
                Lit::Int(syn::LitInt::new(1, syn::IntSuffix::None, Span::call_site())),
                ParamVal::Int(1),
                "int literal",
            ),
            (
                Lit::Bool(syn::LitBool {
                    value: true,
                    span: Span::call_site(),
                }),
                ParamVal::Bool(true),
                "bool literal",
            ),
        ];

        for case in cases.into_iter() {
            assert_eq!(
                ParamVal::from(&case.0),
                case.1,
                "Failed the {} test",
                case.2
            );
        }
    }

    #[test]
    fn bool_conversion() {
        let left = ParamVal::Bool(true).try_into();
        let right = Ok(true);
        assert_eq!(left, right)
    }

    #[test]
    fn int_conversion() {
        let left = ParamVal::Int(1).try_into();
        let right = Ok(1);
        assert_eq!(left, right)
    }

    #[test]
    fn str_conversion() {
        let left: Result<String, TryIntoParamValError> = ParamVal::Str("hello".into()).try_into();
        let right: Result<String, TryIntoParamValError> = Ok("hello".into());
        assert_eq!(left, right)
    }
}
