use crate::ParamVal;
use metafor::metafor;

#[metafor(variant = [
    { name: Bool, ty: bool },
    { name: Int, ty: i64 },
    { name: Float, ty: f64 },
    { name: Str, ty: String }
])]
impl From<__variant__ty__> for ParamVal {
    fn from(src: __variant__ty__) -> Self {
        ParamVal::__variant__name__(src)
    }
}

impl From<&str> for ParamVal {
    fn from(src: &str) -> Self {
        ParamVal::Str(src.into())
    }
}
