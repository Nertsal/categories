mod apply;
mod builder;
pub mod constraint;
pub mod find;
mod init;
pub mod axioms;

use super::*;

pub use builder::*;
use constraint::*;
pub use init::*;

pub struct Rule<L: Label> {
    statement: RuleStatement<L>,
}

pub type RuleStatement<L> = Vec<RuleConstruction<L>>;

pub enum RuleConstruction<L: Label> {
    Forall(Constraints<L>),
    Exists(Constraints<L>),
}

pub type Constraints<L> = Vec<Constraint<L>>;

pub enum Constraint<L: Label> {
    /// Require an object to exist
    Object { label: L, tags: Vec<ObjectTag<L>> },
    /// Require a morphism to exist
    Morphism {
        label: L,
        connection: MorphismConnection<L>,
        tags: Vec<MorphismTag<L, L>>,
    },
    /// Require two morphisms to be equal to each other
    Equality(L, L),
    /// Require a triangle to commute, meaning that: g . f = h
    Commute { f: L, g: L, h: L },
}
