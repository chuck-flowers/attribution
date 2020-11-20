mod keys;
mod values;

pub use self::keys::ParamKey;
pub use self::values::ParamVal;
pub use self::values::TryIntoParamValError;
use crate::conversion::FromParameters;
use crate::conversion::FromParametersError;
use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result as ParseResult;
use syn::Ident;
use syn::Token;

/// Represents the mapping of parameter names to parameter values.
#[derive(Debug, Default)]
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
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let mut params = Parameters::default();

        let mut pos = 0;
        while !input.is_empty() {
            let (key, val) = parse_kv(input)?;
            let key = key
                .map(|i| ParamKey::Named(i.to_string()))
                .unwrap_or_else(|| ParamKey::Unnamed(pos));
            params.insert(key, val);

            if input.peek(Token!(,)) {
                input.parse::<Token!(,)>()?;
            }

            pos += 1;
        }

        Ok(params)
    }
}

fn parse_kv(input: ParseStream) -> ParseResult<(Option<Ident>, ParamVal)> {
    match input.parse::<Ident>() {
        Ok(i) => match input.parse::<Token!(=)>() {
            Ok(_) => {
                let val = input.parse()?;
                Ok((Some(i), val))
            }

            Err(_) => Ok((Some(i), ParamVal::Bool(true))),
        },
        Err(_) => {
            let val = input.parse()?;
            Ok((None, val))
        }
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
    fn from_parameters<'a>(
        params: &mut Parameters,
        _: &'a ParamKey,
    ) -> Result<Self, FromParametersError<'a>> {
        let mut ret = DynamicParameters(Parameters::default());
        let keys: Vec<_> = params.0.keys().cloned().collect();
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
    use pretty_assertions::assert_eq;
    use syn::parse2;
    use syn::parse_quote;
    use syn::Attribute;

    #[test]
    fn parse() {
        let attr: Attribute =
            parse_quote!(#[attr(string = "fooValue", integer = 1, flag = true, simple_flag)]);
        if let proc_macro2::TokenTree::Group(group) = attr.tokens.into_iter().next().unwrap() {
            let attr_args: Parameters = parse2(group.stream()).unwrap();
            let string_val = attr_args.get(&"string".into());
            let integer_val = attr_args.get(&"integer".into());
            let flag_val = attr_args.get(&"flag".into());
            let simple_flag_val = attr_args.get(&"simple_flag".into());
            let other_val = attr_args.get(&"other".into());

            assert_eq!(string_val, Some(&ParamVal::Str("fooValue".to_string())));
            assert_eq!(integer_val, Some(&ParamVal::Int(1)));
            assert_eq!(flag_val, Some(&ParamVal::Bool(true)));
            assert_eq!(simple_flag_val, Some(&ParamVal::Bool(true)));
            assert_eq!(other_val, None);
        } else {
            panic!("Didn't unwrap appropriately");
        }
    }
}
