use std::convert::TryInto;
use syn::Lit;

/// Represents a value for a parameter name within `Parameters` struct.
/// The parameter value is the value that appears to the right of the equal
/// sign (e.g. `"value"` is the `ParamVal` in the following example
/// `#[example(name = "value")]`)
#[derive(Debug, PartialEq)]
pub enum ParamVal {
    Bool(bool),
    SignedInt(i64),
    UnsignedInt(u64),
    Float(f64),
    Str(String),
}

impl From<&Lit> for ParamVal {
    fn from(lit: &Lit) -> Self {
        match lit {
            Lit::Bool(b) => ParamVal::Bool(b.value),
            Lit::Int(i) => {
                if let Ok(unsigned) = i.base10_parse::<u64>() {
                    Self::UnsignedInt(unsigned)
                } else if let Ok(signed) = i.base10_parse::<i64>() {
                    Self::SignedInt(signed)
                } else {
                    panic!(
                        "The integer literal {} could not be parsed as an u64 or an i64",
                        i.base10_digits()
                    );
                }
            }
            Lit::Float(f) => Self::Float(f.base10_parse::<f64>().unwrap()),
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

/// An error that is received as a result of not being able to convert a `ParamVal`
/// into a given type. This is due to the `ParmaVal` not being of the correct variant
/// for the type that the `ParamVal` is being converted into.
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
        if let ParamVal::UnsignedInt(i) = self {
            Ok(i)
        } else {
            Err(TryIntoParamValError {})
        }
    }
}

impl TryInto<u32> for ParamVal {
    type Error = TryIntoParamValError;
    fn try_into(self) -> Result<u32, Self::Error> {
        self.try_into().map(|i: u64| i as u32)
    }
}

impl TryInto<u16> for ParamVal {
    type Error = TryIntoParamValError;
    fn try_into(self) -> Result<u16, Self::Error> {
        self.try_into().map(|i: u64| i as u16)
    }
}

impl TryInto<u8> for ParamVal {
    type Error = TryIntoParamValError;
    fn try_into(self) -> Result<u8, Self::Error> {
        self.try_into().map(|i: u64| i as u8)
    }
}

impl TryInto<i64> for ParamVal {
    type Error = TryIntoParamValError;
    fn try_into(self) -> Result<i64, Self::Error> {
        if let Self::SignedInt(i) = self {
            Ok(i)
        } else {
            Err(Self::Error {})
        }
    }
}

impl TryInto<i32> for ParamVal {
    type Error = TryIntoParamValError;
    fn try_into(self) -> Result<i32, Self::Error> {
        self.try_into().map(|i: i64| i as i32)
    }
}

impl TryInto<i16> for ParamVal {
    type Error = TryIntoParamValError;
    fn try_into(self) -> Result<i16, Self::Error> {
        self.try_into().map(|i: i64| i as i16)
    }
}

impl TryInto<i8> for ParamVal {
    type Error = TryIntoParamValError;
    fn try_into(self) -> Result<i8, Self::Error> {
        self.try_into().map(|i: i64| i as i8)
    }
}

impl TryInto<f64> for ParamVal {
    type Error = TryIntoParamValError;
    fn try_into(self) -> Result<f64, Self::Error> {
        if let ParamVal::Float(f) = self {
            Ok(f)
        } else {
            Err(Self::Error {})
        }
    }
}

impl TryInto<f32> for ParamVal {
    type Error = TryIntoParamValError;
    fn try_into(self) -> Result<f32, Self::Error> {
        self.try_into().map(|f: f64| f as f32)
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
        ParamVal::UnsignedInt(src)
    }
}

impl From<bool> for ParamVal {
    fn from(src: bool) -> Self {
        ParamVal::Bool(src)
    }
}

#[cfg(test)]
mod tests {

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
                Lit::Int(syn::LitInt::new("1", Span::call_site())),
                ParamVal::UnsignedInt(1),
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
    fn unsigned_int_conversion() {
        let left: Result<u32, _> = ParamVal::UnsignedInt(1).try_into();
        let right: Result<u32, _> = Ok(1);
        assert_eq!(left, right)
    }

    #[test]
    fn signed_int_conversion() {
        let left: Result<i32, _> = ParamVal::SignedInt(-1).try_into();
        let right: Result<i32, _> = Ok(-1);
        assert_eq!(left, right)
    }

    #[test]
    fn str_conversion() {
        let left: Result<String, TryIntoParamValError> = ParamVal::Str("hello".into()).try_into();
        let right: Result<String, TryIntoParamValError> = Ok("hello".into());
        assert_eq!(left, right)
    }
}
