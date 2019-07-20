use crate::attr_map::AttrVal;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct TryIntoAttrValError;

impl TryInto<bool> for AttrVal {
    type Error = TryIntoAttrValError;
    fn try_into(self) -> Result<bool, Self::Error> {
        if let AttrVal::Bool(b) = self {
            Ok(b)
        } else {
            Err(TryIntoAttrValError {})
        }
    }
}

impl TryInto<u64> for AttrVal {
    type Error = TryIntoAttrValError;
    fn try_into(self) -> Result<u64, Self::Error> {
        if let AttrVal::Int(i) = self {
            Ok(i)
        } else {
            Err(TryIntoAttrValError {})
        }
    }
}

impl TryInto<String> for AttrVal {
    type Error = TryIntoAttrValError;
    fn try_into(self) -> Result<String, Self::Error> {
        if let AttrVal::Str(s) = self {
            Ok(s)
        } else {
            Err(TryIntoAttrValError {})
        }
    }
}

impl From<&str> for AttrVal {
    fn from(src: &str) -> Self {
        AttrVal::Str(src.into())
    }
}

impl From<u64> for AttrVal {
    fn from(src: u64) -> Self {
        AttrVal::Int(src)
    }
}

impl From<bool> for AttrVal {
    fn from(src: bool) -> Self {
        AttrVal::Bool(src)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attr_map::AttrVal;

    #[test]
    fn bool_conversion() {
        let left = AttrVal::Bool(true).try_into();
        let right = Ok(true);
        assert_eq!(left, right)
    }

    #[test]
    fn int_conversion() {
        let left = AttrVal::Int(1).try_into();
        let right = Ok(1);
        assert_eq!(left, right)
    }

    #[test]
    fn str_conversion() {
        let left: Result<String, TryIntoAttrValError> = AttrVal::Str("hello".into()).try_into();
        let right: Result<String, TryIntoAttrValError> = Ok("hello".into());
        assert_eq!(left, right)
    }
}
