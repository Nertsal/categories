use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RuleObject<O = Label, M = Label> {
    Object { tag: Option<ObjectTag<Option<O>>> },
    Morphism { constraint: ArrowConstraint<O, M> },
}

impl<O, M> RuleObject<O, M> {
    pub fn object(tag: Option<ObjectTag<Option<O>>>) -> Self {
        Self::Object { tag }
    }

    pub fn morphism(
        connection: MorphismConnection<O>,
        tag: Option<MorphismTag<Option<O>, Option<M>>>,
    ) -> Self {
        Self::Morphism {
            constraint: ArrowConstraint { connection, tag },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArrowConstraint<V = Label, E = Label> {
    pub connection: MorphismConnection<V>,
    pub tag: Option<MorphismTag<Option<V>, Option<E>>>,
}
