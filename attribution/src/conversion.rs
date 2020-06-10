use crate::params::ParamKey;
use crate::params::ParamVal;
use crate::Parameters;
use core::convert::TryFrom;
use core::convert::TryInto;
use proc_macro2::Span as Span2;
use syn::parse::Error as ParseError;

/// An error that occurs as a result of a failed conversion of a `Parameters`
/// struct
#[derive(Debug)]
pub enum FromParametersError<'a> {
    /// Indicates the error ocurred because a value for a specified parameter
    /// was not supplied.
    MissingParam { param_key: &'a ParamKey },

    /// Indicates the error occurred because the value that was attempted for conversion
    /// was for the incorrect type.
    UnexpectedType,
}

impl<'a> From<FromParametersError<'a>> for ParseError {
    fn from(src: FromParametersError<'a>) -> ParseError {
        match src {
            FromParametersError::MissingParam { param_key } => {
                let message = format!("The required parameter '{}' is missing.", param_key);
                ParseError::new(Span2::call_site(), message)
            }
            FromParametersError::UnexpectedType => {
                let message = "An unexpected type was provided to a parameter.";
                ParseError::new(Span2::call_site(), message)
            }
        }
    }
}

/// A trait that is used to extract data from a `Parameters` struct.
pub trait FromParameters: Sized {
    /// Try to create a type from a parameter struct (`params`) for a paramter
    /// of a specific name (`param_name`).
    fn from_parameters<'a>(
        params: &mut Parameters,
        param_key: &'a ParamKey,
    ) -> Result<Self, FromParametersError<'a>>;
}

impl<T> FromParameters for T
where
    T: TryFrom<ParamVal>,
{
    fn from_parameters<'a>(
        params: &mut Parameters,
        param_key: &'a ParamKey,
    ) -> Result<Self, FromParametersError<'a>> {
        if let Some(param_val) = params.remove(param_key) {
            T::try_from(param_val).map_err(|_| FromParametersError::UnexpectedType)
        } else {
            Err(FromParametersError::MissingParam { param_key })
        }
    }
}

impl FromParameters for Option<bool> {
    fn from_parameters<'a>(
        params: &mut Parameters,
        param_key: &'a ParamKey,
    ) -> Result<Self, FromParametersError<'a>> {
        Ok(params
            .remove(param_key)
            .map(|val| val.try_into().ok())
            .flatten())
    }
}

impl FromParameters for Option<i64> {
    fn from_parameters<'a>(
        params: &mut Parameters,
        param_key: &'a ParamKey,
    ) -> Result<Self, FromParametersError<'a>> {
        Ok(params
            .remove(param_key)
            .map(|val| val.try_into().ok())
            .flatten())
    }
}

impl FromParameters for Option<f64> {
    fn from_parameters<'a>(
        params: &mut Parameters,
        param_key: &'a ParamKey,
    ) -> Result<Self, FromParametersError<'a>> {
        Ok(params
            .remove(param_key)
            .map(|val| val.try_into().ok())
            .flatten())
    }
}

impl FromParameters for Option<String> {
    fn from_parameters<'a>(
        params: &mut Parameters,
        param_key: &'a ParamKey,
    ) -> Result<Self, FromParametersError<'a>> {
        Ok(params
            .remove(param_key)
            .map(|val| val.try_into().ok())
            .flatten())
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
        let param_key = "foo".into();
        let output = bool::from_parameters(&mut params, &param_key);

        assert_eq!(output.unwrap(), true);
    }

    #[test]
    fn from_parameters_str() {
        let mut params = Parameters::default();
        params.insert("foo".into(), ParamVal::Int(1));
        let param_key = "foo".into();
        let output = i64::from_parameters(&mut params, &param_key);

        assert_eq!(output.unwrap(), 1);
    }

    #[test]
    fn from_parameters_int() {
        let mut params = Parameters::default();
        params.insert("foo".into(), ParamVal::Str("bar".into()));
        let param_key = "foo".into();
        let output = String::from_parameters(&mut params, &param_key);

        let right: String = "bar".into();
        assert_eq!(output.unwrap(), right);
    }

    #[test]
    fn from_parameters_bool_option() {
        let mut params = Parameters::default();
        params.insert("foo".into(), ParamVal::Bool(true));
        let param_key = "foo".into();
        let output = Option::<bool>::from_parameters(&mut params, &param_key);

        assert_eq!(output.unwrap(), Some(true));
        let no_output = Option::<bool>::from_parameters(&mut params, &param_key);
        assert_eq!(no_output.unwrap(), None);
    }

    #[test]
    fn from_parameters_str_option() {
        let mut params = Parameters::default();
        params.insert("foo".into(), ParamVal::Int(1));
        let param_key = "foo".into();
        let output = Option::<i64>::from_parameters(&mut params, &param_key);

        assert_eq!(output.unwrap(), Some(1));
        let no_output = Option::<i64>::from_parameters(&mut params, &param_key);
        assert_eq!(no_output.unwrap(), None);
    }

    #[test]
    fn from_parameters_int_option() {
        let mut params = Parameters::default();
        params.insert("foo".into(), "bar".into());
        let param_key = "foo".into();
        let output = Option::<String>::from_parameters(&mut params, &param_key);

        let right: String = "bar".into();
        assert_eq!(output.unwrap(), Some(right));
        let no_output = Option::<String>::from_parameters(&mut params, &param_key);
        assert_eq!(no_output.unwrap(), None);
    }
}
