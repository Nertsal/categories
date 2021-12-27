use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RuleObject<O = Label, M = Label> {
    Vertex { tags: Vec<ObjectTag<Option<O>>> },
    Edge { constraint: ArrowConstraint<O, M> },
}

impl<O, M> RuleObject<O, M> {
    pub fn vertex(tags: Vec<ObjectTag<Option<O>>>) -> Self {
        Self::Vertex { tags }
    }

    pub fn edge(from: O, to: O, tags: Vec<MorphismTag<Option<O>, Option<M>>>) -> Self {
        Self::Edge {
            constraint: ArrowConstraint::new(from, to, tags),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArrowConstraint<V = Label, E = Label> {
    pub from: V,
    pub to: V,
    pub tags: Vec<MorphismTag<Option<V>, Option<E>>>,
}

impl<V, E> ArrowConstraint<V, E> {
    pub fn new(from: V, to: V, tags: Vec<MorphismTag<Option<V>, Option<E>>>) -> Self {
        Self { from, to, tags }
    }
}
