use crate::params::ParamVal;
use crate::Parameters;

#[derive(Debug)]
pub enum FromParameterValueError {
    UnexpectedType,
}

pub trait FromParameterValue: Sized {
    fn from_parameter_value(parameter_val: ParamVal) -> Result<Self, FromParameterValueError>;
}

impl FromParameterValue for u64 {
    fn from_parameter_value(parameter_val: ParamVal) -> Result<Self, FromParameterValueError> {
        if let ParamVal::Int(val) = parameter_val {
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

#[derive(Debug)]
pub enum FromParametersError {
    MissingParam { param_name: String },
    UnexpectedType,
}

pub trait FromParameters: Sized {
    fn from_parameters(
        params: &mut Parameters,
        param_name: &str,
    ) -> Result<Self, FromParametersError>;
}

impl<T: FromParameterValue> FromParameters for T {
    fn from_parameters(
        params: &mut Parameters,
        param_name: &str,
    ) -> Result<Self, FromParametersError> {
        if let Some(parameter_val) = params.remove(param_name) {
            T::from_parameter_value(parameter_val).map_err(|err| match err {
                FromParameterValueError::UnexpectedType => FromParametersError::UnexpectedType,
            })
        } else {
            Err(FromParametersError::MissingParam {
                param_name: param_name.into(),
            })
        }
    }
}

impl<T: FromParameters> FromParameters for Option<T> {
    fn from_parameters(
        params: &mut Parameters,
        param_name: &str,
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
        let mut params = Parameters::new();
        params.insert("foo".into(), ParamVal::Bool(true));
        let output = bool::from_parameters(&mut params, "foo");

        assert_eq!(output.unwrap(), true);
    }

    #[test]
    fn from_parameters_str() {
        let mut params = Parameters::new();
        params.insert("foo".into(), ParamVal::Int(1));
        let output = u64::from_parameters(&mut params, "foo");

        assert_eq!(output.unwrap(), 1);
    }

    #[test]
    fn from_parameters_int() {
        let mut params = Parameters::new();
        params.insert("foo".into(), ParamVal::Str("bar".into()));
        let output = String::from_parameters(&mut params, "foo");

        let right: String = "bar".into();
        assert_eq!(output.unwrap(), right);
    }

    #[test]
    fn from_parameters_bool_option() {
        let mut params = Parameters::new();
        params.insert("foo".into(), ParamVal::Bool(true));
        let output = Option::<bool>::from_parameters(&mut params, "foo");

        assert_eq!(output.unwrap(), Some(true));
        let no_output = Option::<bool>::from_parameters(&mut params, "foo");
        assert_eq!(no_output.unwrap(), None);
    }

    #[test]
    fn from_parameters_str_option() {
        let mut params = Parameters::new();
        params.insert("foo".into(), ParamVal::Int(1));
        let output = Option::<u64>::from_parameters(&mut params, "foo");

        assert_eq!(output.unwrap(), Some(1));
        let no_output = Option::<u64>::from_parameters(&mut params, "foo");
        assert_eq!(no_output.unwrap(), None);
    }

    #[test]
    fn from_parameters_int_option() {
        let mut params = Parameters::new();
        params.insert("foo".into(), ParamVal::Str("bar".into()));
        let output = Option::<String>::from_parameters(&mut params, "foo");

        let right: String = "bar".into();
        assert_eq!(output.unwrap(), Some(right));
        let no_output = Option::<String>::from_parameters(&mut params, "foo");
        assert_eq!(no_output.unwrap(), None);
    }
}
