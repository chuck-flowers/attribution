use std::collections::HashMap;
use syn::parse::Parse;
use syn::parse::Result;
use syn::parse_macro_input::ParseMacroInput;
use syn::Lit;
use syn::Meta;
use syn::NestedMeta;

pub struct AttrMap(pub HashMap<String, AttrVal>);

impl AttrMap {
    pub fn new() -> Self {
        AttrMap(HashMap::new())
    }

    pub fn get(&self, key: &str) -> Option<&AttrVal> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: String, val: AttrVal) -> Option<AttrVal> {
        self.0.insert(key, val)
    }
}

impl Parse for AttrMap {
    fn parse(buffer: &syn::parse::ParseBuffer<'_>) -> Result<Self> {
        println!("AttrMap::parse({:?})", buffer);
        let mut attribute_map = Self::new();

        let args = syn::AttributeArgs::parse(buffer)?;
        for arg in args.iter() {
            println!("for loop iter");
            if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
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

    #[test]
    fn parse_test() {
        let attr: syn::Attribute = syn::parse_quote!(#[attr foo = "fooValue", bar = 1, baz = true]);
        let attr_args_stream = attr.tts.into();
        let attr_args: AttrMap = syn::parse2(attr_args_stream).unwrap();

        let foo_val: Option<&AttrVal> = attr_args.get("foo");
        let bar_val: Option<&AttrVal> = attr_args.get("bar");
        let baz_val: Option<&AttrVal> = attr_args.get("baz");
        let other_val: Option<&AttrVal> = attr_args.get("other");

        assert_eq!(foo_val, Some(&AttrVal::Str("fooValue".to_string())));
        assert_eq!(bar_val, Some(&AttrVal::Integer(1)));
        assert_eq!(baz_val, Some(&AttrVal::Bool(true)));
        assert_eq!(other_val, None);
    }
}

#[derive(Debug, PartialEq)]
pub enum AttrVal {
    Bool(bool),
    Integer(u64),
    Str(String),
}

impl From<&Lit> for AttrVal {
    fn from(lit: &Lit) -> Self {
        match lit {
            Lit::Bool(b) => AttrVal::Bool(b.value),
            Lit::Int(i) => AttrVal::Integer(i.value()),
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
                AttrVal::Integer(1),
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
