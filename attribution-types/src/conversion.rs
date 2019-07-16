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
        if let AttrVal::Integer(i) = self {
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
        let left = AttrVal::Integer(1).try_into();
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
