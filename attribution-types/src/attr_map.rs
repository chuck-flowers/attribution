use std::collections::HashMap;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;
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
    fn parse(buffer: ParseStream) -> Result<Self> {
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
}
