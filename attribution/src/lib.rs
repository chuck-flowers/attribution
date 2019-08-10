#![warn(clippy::all, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

mod conversion;
mod params;

pub use attribution_macros::attr_args;
pub use conversion::FromParameterValue;
pub use conversion::FromParameterValueError;
pub use conversion::FromParameters;
pub use conversion::FromParametersError;
pub use params::DynamicParameters;
pub use params::ParamVal;
pub use params::Parameters;
