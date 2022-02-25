use super::*;

pub struct RuleBuilder<L: Label> {
    statement: RuleStatement<L>,
}

impl<L: Label> RuleBuilder<L> {
    pub fn new() -> Self {
        Self { statement: vec![] }
    }

    pub fn forall(mut self, constraints: impl Into<Constraints<L>>) -> Self {
        self.statement
            .push(RuleConstruction::Forall(constraints.into()));
        self
    }

    pub fn exists(mut self, constraints: impl Into<Constraints<L>>) -> Self {
        self.statement
            .push(RuleConstruction::Exists(constraints.into()));
        self
    }

    pub fn build(self) -> Result<Rule<L>, RuleConstructionError> {
        Rule::new(self.statement)
    }
}
