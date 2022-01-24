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

    pub fn object(mut self, label: &str, tag: Option<ObjectTag<Option<&str>>>) -> Self {
        self.0.push(Constraint::RuleObject(
            Label::Name(label.to_owned()),
            RuleObject::object(tag.map(|tag| tag.map(|label| label.map(|label| label.into())))),
        ));
        self
    }

    pub fn morphism(
        mut self,
        label: &str,
        from: impl Into<Label>,
        to: impl Into<Label>,
        tag: Option<MorphismTag<Option<&str>, Option<&str>>>,
    ) -> Self {
        self.0.push(Constraint::RuleObject(
            Label::Name(label.to_owned()),
            RuleObject::morphism(
                MorphismConnection::Regular {
                    from: from.into(),
                    to: to.into(),
                },
                tag.map(|tag| {
                    tag.map(
                        |label| label.map(|label| label.into()),
                        |label| label.map(|label| label.into()),
                    )
                }),
            ),
        ));
        self
    }

    pub fn isomorphism(
        mut self,
        label: &str,
        object_a: impl Into<Label>,
        object_b: impl Into<Label>,
        tag: Option<MorphismTag<Option<&str>, Option<&str>>>,
    ) -> Self {
        self.0.push(Constraint::RuleObject(
            Label::Name(label.to_owned()),
            RuleObject::morphism(
                MorphismConnection::Isomorphism(object_a.into(), object_b.into()),
                tag.map(|tag| {
                    tag.map(
                        |label| label.map(|label| label.into()),
                        |label| label.map(|label| label.into()),
                    )
                }),
            ),
        ));
        self
    }

    pub fn equality(mut self, label_f: &str, label_g: &str) -> Self {
        self.0.push(Constraint::MorphismEq(
            Label::Name(label_f.to_owned()),
            Label::Name(label_g.to_owned()),
        ));
        self
    }
}
