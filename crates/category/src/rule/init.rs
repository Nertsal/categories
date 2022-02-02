use super::*;

#[derive(Debug, Clone)]
pub enum RuleConstructionError {}

impl<L: Label> Rule<L> {
    pub fn new(statement: RuleStatement<L>) -> Result<Self, RuleConstructionError> {
        // TODO: check that the statement is valid
        Ok(Self { statement })
    }
}
