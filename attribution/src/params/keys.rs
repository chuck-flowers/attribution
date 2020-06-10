use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

/// Represents an identifier for a `ParamVal`
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ParamKey {
    Named(String),
    Unnamed(usize),
}

impl Display for ParamKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ParamKey::Named(name) => name.fmt(f),
            ParamKey::Unnamed(pos) => pos.fmt(f),
        }
    }
}

impl From<String> for ParamKey {
    fn from(src: String) -> Self {
        ParamKey::Named(src)
    }
}

impl From<&str> for ParamKey {
    fn from(src: &str) -> Self {
        ParamKey::Named(src.into())
    }
}

impl From<usize> for ParamKey {
    fn from(src: usize) -> Self {
        ParamKey::Unnamed(src)
    }
}
