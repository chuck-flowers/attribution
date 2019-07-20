use std::collections::HashMap;
use syn::parse::Parse;
use syn::parse::Result;
use syn::Lit;
use syn::Meta;
use syn::NestedMeta;

pub struct AttrMap(HashMap<String, AttrVal>);

impl AttrMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<&AttrVal> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: String, val: AttrVal) -> Option<AttrVal> {
        self.0.insert(key, val)
    }

    pub fn remove(&mut self, key: &str) -> Option<AttrVal> {
        self.0.remove(key)
    }
}

impl Default for AttrMap {
    fn default() -> Self {
        AttrMap(HashMap::new())
    }
}

impl Parse for AttrMap {
    fn parse(buffer: &syn::parse::ParseBuffer<'_>) -> Result<Self> {
        let mut attribute_map = Self::new();

        while !buffer.is_empty() {
            if let NestedMeta::Meta(Meta::NameValue(nv)) = buffer.parse()? {
                let param_name = nv.ident.to_string();
                let param_value = AttrVal::from(&nv.lit);
                attribute_map.insert(param_name, param_value);
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
        dbg!(&attr.tts);
        let attr_args: AttrMap = parse2(attr.tts).unwrap();

        let foo_val = attr_args.get("foo");
        let bar_val = attr_args.get("bar");
        let baz_val = attr_args.get("baz");
        let other_val = attr_args.get("other");

        assert_eq!(foo_val, Some(&AttrVal::Str("fooValue".to_string())));
        assert_eq!(bar_val, Some(&AttrVal::Int(1)));
        assert_eq!(baz_val, Some(&AttrVal::Bool(true)));
        assert_eq!(other_val, None);
    }
}

#[derive(Debug, PartialEq)]
pub enum AttrVal {
    Bool(bool),
    Int(u64),
    Str(String),
}

impl From<&Lit> for AttrVal {
    fn from(lit: &Lit) -> Self {
        match lit {
            Lit::Bool(b) => AttrVal::Bool(b.value),
            Lit::Int(i) => AttrVal::Int(i.value()),
            Lit::Str(s) => AttrVal::Str(s.value()),
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
                AttrVal::Str("foo".to_string()),
                "string literal",
            ),
            (
                Lit::Int(syn::LitInt::new(1, syn::IntSuffix::None, Span::call_site())),
                AttrVal::Int(1),
                "int literal",
            ),
            (
                Lit::Bool(syn::LitBool {
                    value: true,
                    span: Span::call_site(),
                }),
                AttrVal::Bool(true),
                "bool literal",
            ),
        ];

        for case in cases.into_iter() {
            assert_eq!(AttrVal::from(&case.0), case.1, "Failed the {} test", case.2);
        }
    }
}
