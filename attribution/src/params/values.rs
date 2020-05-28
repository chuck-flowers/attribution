mod convert;
mod parsing;

pub use self::convert::TryIntoParamValError;

/// Represents a value for a parameter name within `Parameters` struct.
/// The parameter value is the value that appears to the right of the equal
/// sign (e.g. `"value"` is the `ParamVal` in the following example
/// `#[example(name = "value")]`)
#[derive(Debug, PartialEq)]
pub enum ParamVal {
    Array(Vec<ParamVal>),
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
}
