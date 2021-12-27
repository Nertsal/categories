use super::*;

#[derive(Debug)]
pub enum RuleConstruction {
    Forall(Constraints),
    Exists(Constraints),
}

pub type Constraints = Vec<Constraint>;

#[derive(Debug, Clone)]
pub enum Constraint {
    RuleObject(Label, RuleObject),
    MorphismEq(Label, Label),
}

pub struct ConstraintsBuilder(Constraints);

impl Into<Constraints> for ConstraintsBuilder {
    fn into(self) -> Constraints {
        self.build()
    }
}

impl ConstraintsBuilder {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn build(self) -> Constraints {
        self.0
    }

    pub fn object(mut self, label: impl Into<Label>, tags: Vec<ObjectTag>) -> Self {
        self.0.push(Constraint::RuleObject(
            label.into(),
            RuleObject::vertex(tags),
        ));
        self
    }

    pub fn morphism(
        mut self,
        label: impl Into<Label>,
        from: impl Into<Label>,
        to: impl Into<Label>,
        tags: Vec<MorphismTag>,
    ) -> Self {
        self.0.push(Constraint::RuleObject(
            label.into(),
            RuleObject::edge(from.into(), to.into(), tags),
        ));
        self
    }
}
