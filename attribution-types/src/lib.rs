#![warn(clippy::all)]

mod attr_map;
mod conversion;

pub use attr_map::ParamVal;
pub use attr_map::Parameters;
pub use conversion::TryIntoParamValError;
