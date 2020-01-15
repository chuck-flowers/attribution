use crate::params::ParamKey;
use crate::params::ParamVal;
use crate::Parameters;

/// An error that occurs as a result of a failed conversion of a `ParameterVal`
#[derive(Debug)]
pub enum FromParameterValueError {
    /// The data that was being attempted for conversion was not of the expected type.
    UnexpectedType,
}

/// A trait that is used to create a type from a provided `ParamVal`. This
/// trait should be implemented when a type should be able to be converted from
/// a single `ParamVal` type.
///
/// # Example
/// ```
/// use attribution::FromParameterValue;
/// use attribution::ParamVal;
///
/// let param_val = ParamVal::Bool(true);
/// let from_result = bool::from_parameter_value(param_val);
/// assert_eq!(from_result.unwrap(), true);
/// ```
pub trait FromParameterValue: Sized {
    /// Tries to create a type from the provided `ParamVal`
    fn from_parameter_value(parameter_val: ParamVal) -> Result<Self, FromParameterValueError>;
}

impl FromParameterValue for u64 {
    fn from_parameter_value(parameter_val: ParamVal) -> Result<Self, FromParameterValueError> {
        if let ParamVal::UnsignedInt(val) = parameter_val {
            Ok(val)
        } else {
            Err(FromParameterValueError::UnexpectedType)
        }
    }
}

impl FromParameterValue for bool {
    fn from_parameter_value(parameter_val: ParamVal) -> Result<Self, FromParameterValueError> {
        if let ParamVal::Bool(val) = parameter_val {
            Ok(val)
        } else {
            Err(FromParameterValueError::UnexpectedType)
        }
    }
}

impl FromParameterValue for String {
    fn from_parameter_value(parameter_val: ParamVal) -> Result<Self, FromParameterValueError> {
        if let ParamVal::Str(val) = parameter_val {
            Ok(val)
        } else {
            Err(FromParameterValueError::UnexpectedType)
        }
    }
}

/// An error that occurs as a result of a failed conversion of a `Parameters`
/// struct
#[derive(Debug)]
pub enum FromParametersError {
    /// Indicates the error ocurred because a value for a specified parameter
    /// was not supplied.
    MissingParam { param_name: ParamKey },

    /// Indicates the error occurred because the value that was attempted for conversion
    /// was for the incorrect type.
    UnexpectedType,
}

/// A trait that is used to extract data from a `Parameters` struct.
pub trait FromParameters: Sized {
    /// Try to create a type from a parameter struct (`params`) for a paramter
    /// of a specific name (`param_name`).
    fn from_parameters(
        params: &mut Parameters,
        param_name: &ParamKey,
    ) -> Result<Self, FromParametersError>;
}

impl<T: FromParameterValue> FromParameters for T {
    fn from_parameters(
        params: &mut Parameters,
        param_name: &ParamKey,
    ) -> Result<Self, FromParametersError> {
        if let Some(parameter_val) = params.remove(&param_name.to_owned().into()) {
            T::from_parameter_value(parameter_val).map_err(|err| match err {
                FromParameterValueError::UnexpectedType => FromParametersError::UnexpectedType,
            })
        } else {
            Err(FromParametersError::MissingParam {
                param_name: param_name.clone(),
            })
        }
    }
}

impl<T: FromParameters> FromParameters for Option<T> {
    fn from_parameters(
        params: &mut Parameters,
        param_name: &ParamKey,
    ) -> Result<Self, FromParametersError> {
        Ok(T::from_parameters(params, param_name).ok())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::Parameters;

    #[test]
    fn from_parameters_bool() {
        let mut params = Parameters::default();
        params.insert("foo".into(), ParamVal::Bool(true));
        let output = bool::from_parameters(&mut params, &"foo".into());

        assert_eq!(output.unwrap(), true);
    }

    #[test]
    fn from_parameters_str() {
        let mut params = Parameters::default();
        params.insert("foo".into(), ParamVal::UnsignedInt(1));
        let output = u64::from_parameters(&mut params, &"foo".into());

        assert_eq!(output.unwrap(), 1);
    }

    #[test]
    fn from_parameters_int() {
        let mut params = Parameters::default();
        params.insert("foo".into(), ParamVal::Str("bar".into()));
        let output = String::from_parameters(&mut params, &"foo".into());

        let right: String = "bar".into();
        assert_eq!(output.unwrap(), right);
    }

    #[test]
    fn from_parameters_bool_option() {
        let mut params = Parameters::default();
        params.insert("foo".into(), ParamVal::Bool(true));
        let output = Option::<bool>::from_parameters(&mut params, &"foo".into());

        assert_eq!(output.unwrap(), Some(true));
        let no_output = Option::<bool>::from_parameters(&mut params, &"foo".into());
        assert_eq!(no_output.unwrap(), None);
    }

    #[test]
    fn from_parameters_str_option() {
        let mut params = Parameters::default();
        params.insert("foo".into(), ParamVal::UnsignedInt(1));
        let output = Option::<u64>::from_parameters(&mut params, &"foo".into());

        assert_eq!(output.unwrap(), Some(1));
        let no_output = Option::<u64>::from_parameters(&mut params, &"foo".into());
        assert_eq!(no_output.unwrap(), None);
    }

    #[test]
    fn from_parameters_int_option() {
        let mut params = Parameters::default();
        params.insert("foo".into(), ParamVal::Str("bar".into()));
        let output = Option::<String>::from_parameters(&mut params, &"foo".into());

        let right: String = "bar".into();
        assert_eq!(output.unwrap(), Some(right));
        let no_output = Option::<String>::from_parameters(&mut params, &"foo".into());
        assert_eq!(no_output.unwrap(), None);
    }
}
