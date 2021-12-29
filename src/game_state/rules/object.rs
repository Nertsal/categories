use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RuleObject<O = Label, M = Label> {
    Vertex { tag: Option<ObjectTag<Option<O>>> },
    Edge { constraint: ArrowConstraint<O, M> },
}

impl<O, M> RuleObject<O, M> {
    pub fn vertex(tag: Option<ObjectTag<Option<O>>>) -> Self {
        Self::Vertex { tag }
    }

    pub fn edge(from: O, to: O, tag: Option<MorphismTag<Option<O>, Option<M>>>) -> Self {
        Self::Edge {
            constraint: ArrowConstraint::new(from, to, tag),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArrowConstraint<V = Label, E = Label> {
    pub from: V,
    pub to: V,
    pub tag: Option<MorphismTag<Option<V>, Option<E>>>,
}

impl<V, E> ArrowConstraint<V, E> {
    pub fn new(from: V, to: V, tag: Option<MorphismTag<Option<V>, Option<E>>>) -> Self {
        Self { from, to, tag }
    }
}
