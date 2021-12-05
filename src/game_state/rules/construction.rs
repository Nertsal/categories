use super::*;

#[derive(Debug)]
pub enum RuleConstruction {
    Forall(Constraints),
    Exists(Constraints),
}

pub type Constraints = Vec<Constraint>;

#[derive(Debug)]
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

    pub fn object(mut self, label: &str, tags: Vec<ObjectTag<&str>>) -> Self {
        self.0.push(Constraint::RuleObject(
            label.to_owned(),
            RuleObject::vertex(
                tags.into_iter()
                    .map(|tag| tag.map(|o| o.to_owned()))
                    .collect(),
            ),
        ));
        self
    }

    pub fn morphism(
        mut self,
        label: &str,
        from: &str,
        to: &str,
        tags: Vec<MorphismTag<&str, &str>>,
    ) -> Self {
        self.0.push(Constraint::RuleObject(
            label.to_owned(),
            RuleObject::edge(
                from.to_owned(),
                to.to_owned(),
                tags.into_iter()
                    .map(|tag| tag.map(|o| o.to_owned(), |m| m.to_owned()))
                    .collect(),
            ),
        ));
        self
    }
}
