#![warn(clippy::all)]

mod attr_map;
mod conversion;

pub use attr_map::ParamVal;
pub use attr_map::Parameters;
pub use attribution_macros::attr_args;
pub use conversion::TryIntoParamValError;
