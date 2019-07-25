#![warn(clippy::all, clippy::cargo)]

mod conversion;
mod params;

pub use attribution_macros::attr_args;
pub use params::ParamVal;
pub use params::Parameters;
