#![warn(clippy::all, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

mod conversion;
mod params;

pub use attribution_macros::AttrArgs;
pub use conversion::FromParameters;
pub use conversion::FromParametersError;
pub use params::DynamicParameters;
pub use params::ParamVal;
pub use params::Parameters;
