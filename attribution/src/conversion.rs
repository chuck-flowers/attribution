use crate::attr_map::ParamVal;
use crate::Parameters;
use std::convert::TryInto;

pub enum FromParametersError {
    MissingParam { param_name: String },
    UnexpectedType,
}

trait FromParameters: Sized {
    fn from_parameters(
        params: &mut Parameters,
        field_name: &str,
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
