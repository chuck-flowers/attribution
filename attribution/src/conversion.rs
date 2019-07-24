use crate::attr_map::ParamVal;
use crate::Parameters;
use std::convert::TryInto;

pub enum FromParametersError {}

trait FromParameters: Sized {
    fn from_parameters(
        params: &mut Parameters,
        field_name: &str,
    ) -> Result<Self, FromParametersError>;
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
mod tests {
    use super::*;
    use crate::attr_map::ParamVal;

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
