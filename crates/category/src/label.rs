pub trait Label: std::hash::Hash + Eq + Clone {}

impl<T: std::hash::Hash + Eq + Clone> Label for T {}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum StringLabel {
    Name(String),
    Unknown,
}

impl From<&str> for StringLabel {
    fn from(name: &str) -> Self {
        Self::Name(name.to_owned())
    }
}
