use crate::ParamVal;

impl From<bool> for ParamVal {
    fn from(src: bool) -> Self {
        ParamVal::Bool(src)
    }
}

impl From<i64> for ParamVal {
    fn from(src: i64) -> Self {
        ParamVal::Int(src)
    }
}

impl From<f64> for ParamVal {
    fn from(src: f64) -> Self {
        ParamVal::Float(src)
    }
}

impl From<String> for ParamVal {
    fn from(src: String) -> Self {
        ParamVal::Str(src)
    }
}

impl From<&str> for ParamVal {
    fn from(src: &str) -> Self {
        ParamVal::Str(src.into())
    }
}
