#![warn(clippy::all)]

mod attr_map;
mod conversion;

pub use attr_map::AttrMap;
pub use attr_map::ParamVal;
pub use conversion::TryIntoParamValError;
