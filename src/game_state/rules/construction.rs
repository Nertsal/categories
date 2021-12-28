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

    pub fn object(mut self, label: &str, tags: Vec<ObjectTag<Option<&str>>>) -> Self {
        self.0.push(Constraint::RuleObject(
            Label::Name(label.to_owned()),
            RuleObject::vertex(
                tags.into_iter()
                    .map(|tag| tag.map(|label| label.map(|label| label.into())))
                    .collect(),
            ),
        ));
        self
    }

    pub fn morphism(
        mut self,
        label: &str,
        from: impl Into<Label>,
        to: impl Into<Label>,
        tags: Vec<MorphismTag<Option<&str>, Option<&str>>>,
    ) -> Self {
        self.0.push(Constraint::RuleObject(
            Label::Name(label.to_owned()),
            RuleObject::edge(
                from.into(),
                to.into(),
                tags.into_iter()
                    .map(|tag| {
                        tag.map(
                            |label| label.map(|label| label.into()),
                            |label| label.map(|label| label.into()),
                        )
                    })
                    .collect(),
            ),
        ));
        self
    }
}
