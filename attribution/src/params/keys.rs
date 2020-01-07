/// Represents an identifier for a `ParamVal`
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ParamKey {
    Named(String),
    Unnamed(usize),
}

impl From<&str> for ParamKey {
    fn from(src: &str) -> Self {
        ParamKey::Named(src.into())
    }
}

impl From<String> for ParamKey {
    fn from(src: String) -> Self {
        ParamKey::Named(src)
    }
}

impl From<usize> for ParamKey {
    fn from(src: usize) -> Self {
        ParamKey::Unnamed(src)
    }
}
