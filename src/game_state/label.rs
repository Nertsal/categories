#[derive(Debug, Clone)]
pub enum Label {
    Name(String),
    Any,
}

impl Label {
    pub fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (Label::Name(self_name), Label::Name(other_name)) => self_name == other_name,
            _ => true,
        }
    }
}

impl<T> From<T> for Label
where
    T: AsRef<str>,
{
    fn from(label: T) -> Self {
        Self::Name(label.as_ref().to_owned())
    }
}
