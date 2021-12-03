use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RuleObject<T = Label> {
    Vertex,
    Edge { constraint: ArrowConstraint<T> },
}

impl<T> RuleObject<T> {
    pub fn vertex() -> Self {
        Self::Vertex
    }

    pub fn edge(from: T, to: T, connection: ArrowConnection) -> Self {
        Self::Edge {
            constraint: ArrowConstraint::new(from, to, connection),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArrowConstraint<T> {
    pub from: T,
    pub to: T,
    pub connection: ArrowConnection,
}

impl<T> ArrowConstraint<T> {
    pub fn new(from: T, to: T, connection: ArrowConnection) -> Self {
        Self {
            from,
            to,
            connection,
        }
    }
}
