pub trait Label: std::hash::Hash + Eq + Clone {}

impl<T: std::hash::Hash + Eq + Clone> Label for T {}
