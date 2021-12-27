#[derive(Debug, Clone)]
pub enum RuleLabel {
    Name(String),
    Any,
}

impl<T> From<T> for RuleLabel
where
    T: AsRef<str>,
{
    fn from(label: T) -> Self {
        Self::Name(label.as_ref().to_owned())
    }
}
