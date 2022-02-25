pub trait Label: std::fmt::Debug + std::hash::Hash + Eq + Clone + Ord {}

impl<T: std::fmt::Debug + std::hash::Hash + Eq + Clone + Ord> Label for T {}
