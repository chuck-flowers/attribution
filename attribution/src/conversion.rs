use crate::params::ParamVal;
use crate::Parameters;

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

impl FromParameters for bool {
    fn from_parameters(
        params: &mut Parameters,
        param_name: &str,
    ) -> Result<Self, FromParametersError> {
        if let Some(param_val) = params.remove(param_name) {
            if let ParamVal::Bool(b) = param_val {
                Ok(b)
            } else {
                Err(FromParametersError::UnexpectedType)
            }
        } else {
            Err(FromParametersError::MissingParam {
                param_name: param_name.into(),
            })
        }
    }
}

impl FromParameters for u64 {
    fn from_parameters(
        params: &mut Parameters,
        param_name: &str,
    ) -> Result<Self, FromParametersError> {
        if let Some(param_val) = params.remove(param_name) {
            if let ParamVal::Int(i) = param_val {
                Ok(i)
            } else {
                Err(FromParametersError::UnexpectedType)
            }
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

impl FromParameters for String {
    fn from_parameters(
        params: &mut Parameters,
        param_name: &str,
    ) -> Result<Self, FromParametersError> {
        if let Some(param_val) = params.remove(param_name) {
            if let ParamVal::Str(s) = param_val {
                Ok(s)
            } else {
                Err(FromParametersError::UnexpectedType)
            }
        } else {
            Err(FromParametersError::MissingParam {
                param_name: param_name.into(),
            })
        }
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
