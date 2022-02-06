mod apply;
pub mod axioms;
mod builder;
pub mod constraint;
pub mod find;
mod init;

use super::*;

pub use builder::*;
use constraint::*;
pub use init::*;

pub struct Rule<L: Label> {
    statement: RuleStatement<L>,
}

pub type RuleStatement<L> = Vec<RuleConstruction<L>>;

#[derive(Debug, Clone)]
pub enum RuleConstruction<L: Label> {
    Forall(Constraints<L>),
    Exists(Constraints<L>),
}

pub type Constraints<L> = Vec<Constraint<L>>;

#[derive(Debug, Clone)]
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

impl<L: Label> Rule<L> {
    pub fn get_statement(&self) -> &RuleStatement<L> {
        &self.statement
    }

    pub fn get_input(&self) -> &Constraints<L> {
        match self
            .statement
            .first()
            .expect("A rule is expected to be checked for validity during creation")
        {
            RuleConstruction::Forall(constraints) | RuleConstruction::Exists(constraints) => {
                constraints
            }
        }
    }
}