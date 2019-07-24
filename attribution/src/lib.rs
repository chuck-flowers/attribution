#![warn(clippy::all, clippy::cargo)]

mod attr_map;
mod conversion;

pub use attr_map::ParamVal;
pub use attr_map::Parameters;
pub use attribution_macros::attr_args;
pub use conversion::TryIntoParamValError;
