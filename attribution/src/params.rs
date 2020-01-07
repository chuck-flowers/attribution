mod keys;
mod values;

pub use self::keys::ParamKey;
pub use self::values::ParamVal;
use crate::conversion::FromParameters;
use crate::conversion::FromParametersError;
use quote::ToTokens;
use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::Meta;
use syn::NestedMeta;

/// Represents the mapping of parameter names to parameter values.
#[derive(Default)]
pub struct Parameters(HashMap<ParamKey, ParamVal>);

impl Deref for Parameters {
    type Target = HashMap<ParamKey, ParamVal>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Parameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Parse for Parameters {
    fn parse(buffer: ParseStream) -> syn::parse::Result<Self> {
        let mut attribute_map = Self::default();

        let list: Punctuated<NestedMeta, syn::Token![,]> = Punctuated::parse_terminated(buffer)?;

        for (i, el) in list.into_iter().enumerate() {
            let (key, val) = match el {
                NestedMeta::Meta(meta) => {
                    if let Meta::NameValue(nv) = meta {
                        let mut key_name = String::new();
                        for token in nv.path.to_token_stream() {
                            key_name.push_str(&token.to_string());
                        }

                        (key_name.into(), nv.lit.into())
                    } else {
                        continue;
                    }
                }
                NestedMeta::Lit(lit) => (i.into(), lit.into()),
            };

            attribute_map.0.insert(key, val);
        }

        Ok(attribute_map)
    }
}

/// An object that is used to aggregate any remaining parameters into
/// the struct tagged with `attribution::attr_args`.
#[derive(Default)]
pub struct DynamicParameters(Parameters);

impl Deref for DynamicParameters {
    type Target = Parameters;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DynamicParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromParameters for DynamicParameters {
    fn from_parameters(params: &mut Parameters, _: &ParamKey) -> Result<Self, FromParametersError> {
        let mut ret = DynamicParameters(Parameters::default());
        let keys: Vec<_> = params.0.keys().map(|key| key.clone()).collect();
        for key in keys {
            let val = params.0.remove(&key).unwrap();
            (ret.0).0.insert(key, val);
        }

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse2;
    use syn::parse_quote;
    use syn::Attribute;

    #[test]
    fn parse_test() {
        let attr: Attribute = parse_quote!(#[attr(foo = "fooValue", bar = 1, baz = true)]);
        if let proc_macro2::TokenTree::Group(group) = attr.tokens.into_iter().next().unwrap() {
            let attr_args: Parameters = parse2(group.stream()).unwrap();
            let foo_val = attr_args.get(&"foo".into());
            let bar_val = attr_args.get(&"bar".into());
            let baz_val = attr_args.get(&"baz".into());
            let other_val = attr_args.get(&"other".into());

            assert_eq!(foo_val, Some(&ParamVal::Str("fooValue".to_string())));
            assert_eq!(bar_val, Some(&ParamVal::UnsignedInt(1)));
            assert_eq!(baz_val, Some(&ParamVal::Bool(true)));
            assert_eq!(other_val, None);
        } else {
            panic!("Didn't unwrap appropriately");
        }
    }
}
