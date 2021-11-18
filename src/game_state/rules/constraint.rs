use super::*;

#[derive(Debug, Clone, Copy)]
pub enum RuleObject<T> {
    Vertex {
        label: T,
    },
    Edge {
        label: T,
        constraint: ArrowConstraint<T>,
    },
}

impl<T> RuleObject<T> {
    pub fn vertex(label: T) -> Self {
        Self::Vertex { label }
    }

    pub fn edge(label: T, from: T, to: T, connection: ArrowConnection) -> Self {
        Self::Edge {
            label,
            constraint: ArrowConstraint::new(from, to, connection),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
