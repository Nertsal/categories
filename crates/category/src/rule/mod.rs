mod apply;
pub mod constraint;
mod find;
mod init;

use super::*;

use constraint::*;

pub struct Rule<L: Label> {
    statement: RuleStatement<L>,
}

pub type RuleStatement<L> = Vec<RuleConstruction<L>>;

pub enum RuleConstruction<L: Label> {
    Forall(Constraints<L>),
    Exists(Constraints<L>),
}

pub type Constraints<L> = Vec<Constraint<L>>;

pub trait Label: std::hash::Hash + Eq + Clone {}

impl<T: std::hash::Hash + Eq + Clone> Label for T {}

pub enum Constraint<L: Label> {
    /// Require an object to exist
    Object { label: L, tags: Vec<ObjectTag<L>> },
    /// Require a morphism to exist
    Morphism {
        label: L,
        connection: MorphismConnection<L>,
        tags: Vec<MorphismTag<L>>,
    },
    /// Require two morphisms to be equal to each other
    Equality(L, L),
    /// Require a triangle to commute, meaning that: g . f = h
    Commute { f: L, g: L, h: L },
}
