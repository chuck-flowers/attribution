use crate::ParamVal;
use core::convert::TryFrom;
use core::convert::TryInto;
use metafor::metafor;

/// An error that is received as a result of not being able to convert a `ParamVal`
/// into a given type. This is due to the `ParmaVal` not being of the correct variant
/// for the type that the `ParamVal` is being converted into.
#[derive(Debug, PartialEq)]
pub enum TryIntoParamValError {
    UnexpectedType,
}

#[metafor(variant = [
    { name: Bool, ty: bool },
    { name: Float, ty: f64 },
    { name: Int, ty: i64 },
    { name: Str, ty: String }
])]
impl TryFrom<ParamVal> for __variant__ty__ {
    type Error = TryIntoParamValError;

    fn try_from(param_val: ParamVal) -> Result<Self, Self::Error> {
        if let ParamVal::__variant__name__(b) = param_val {
            Ok(b)
        } else {
            Err(TryIntoParamValError::UnexpectedType)
        }
    }
}

impl<T> TryFrom<ParamVal> for Vec<T>
where
    T: TryFrom<ParamVal, Error = TryIntoParamValError>,
{
    type Error = TryIntoParamValError;

    fn try_from(value: ParamVal) -> Result<Self, Self::Error> {
        if let ParamVal::Array(array) = value {
            array
                .into_iter()
                .map(|sub_val| sub_val.try_into())
                .collect()
        } else {
            Err(Self::Error::UnexpectedType)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::convert::TryInto;

    #[test]
    fn array_conversion() {
        let left =
            ParamVal::Array(vec![ParamVal::Int(1), ParamVal::Int(2), ParamVal::Int(3)]).try_into();
        let right = Ok(vec![1, 2, 3]);
        assert_eq!(left, right);
    }

    #[test]
    fn bool_conversion() {
        let left = ParamVal::Bool(true).try_into();
        let right = Ok(true);
        assert_eq!(left, right)
    }

    #[test]
    fn float_conversion() {
        let left: Result<f64, _> = ParamVal::Float(1.0).try_into();
        let right: Result<f64, _> = Ok(1.0);
        assert_eq!(left, right)
    }

    #[test]
    fn int_conversion() {
        let left: Result<i64, _> = ParamVal::Int(-1).try_into();
        let right: Result<i64, _> = Ok(-1);
        assert_eq!(left, right)
    }

    #[test]
    fn str_conversion() {
        let left: Result<String, _> = ParamVal::Str("hello".into()).try_into();
        let right: Result<String, _> = Ok("hello".into());
        assert_eq!(left, right)
    }
}
