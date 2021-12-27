#[derive(Debug, Clone)]
pub enum Label {
    Name(String),
    Any,
}

impl<T> From<T> for Label
where
    T: AsRef<str>,
{
    fn from(label: T) -> Self {
        Self::Name(label.as_ref().to_owned())
    }
}
