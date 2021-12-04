use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RuleObject<O = Label, M = Label> {
    Vertex,
    Edge { constraint: ArrowConstraint<O, M> },
}

impl<O, M> RuleObject<O, M> {
    pub fn vertex() -> Self {
        Self::Vertex
    }

    pub fn edge(from: O, to: O, tags: Vec<MorphismTag<O, M>>) -> Self {
        Self::Edge {
            constraint: ArrowConstraint::new(from, to, tags),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArrowConstraint<V = Label, E = Label> {
    pub from: V,
    pub to: V,
    pub tags: Vec<MorphismTag<V, E>>,
}

impl<V, E> ArrowConstraint<V, E> {
    pub fn new(from: V, to: V, tags: Vec<MorphismTag<V, E>>) -> Self {
        Self { from, to, tags }
    }
}
