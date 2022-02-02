use super::*;

pub struct ConstraintsBuilder<L: Label>(Constraints<L>);

impl<L: Label> Into<Constraints<L>> for ConstraintsBuilder<L> {
    fn into(self) -> Constraints<L> {
        self.build()
    }
}

impl<L: Label> ConstraintsBuilder<L> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn build(self) -> Constraints<L> {
        self.0
    }

    pub fn object(
        mut self,
        label: impl Into<L>,
        tags: impl IntoIterator<Item = ObjectTag<impl Into<L>>>,
    ) -> Self {
        self.0.push(Constraint::Object {
            label: label.into(),
            tags: tags
                .into_iter()
                .map(|tag| tag.map(|label| label.into()))
                .collect(),
        });
        self
    }

    pub fn morphism(
        mut self,
        label: impl Into<L>,
        from: impl Into<L>,
        to: impl Into<L>,
        tags: impl IntoIterator<Item = MorphismTag<impl Into<L>, impl Into<L>>>,
    ) -> Self {
        self.0.push(Constraint::Morphism {
            label: label.into(),
            connection: MorphismConnection::Regular {
                from: from.into(),
                to: to.into(),
            },
            tags: tags
                .into_iter()
                .map(|tag| tag.map(|label| label.into(), |label| label.into()))
                .collect(),
        });
        self
    }

    pub fn isomorphism(
        mut self,
        label: impl Into<L>,
        object_a: impl Into<L>,
        object_b: impl Into<L>,
        tags: impl IntoIterator<Item = MorphismTag<impl Into<L>, impl Into<L>>>,
    ) -> Self {
        self.0.push(Constraint::Morphism {
            label: label.into(),
            connection: MorphismConnection::Isomorphism(object_a.into(), object_b.into()),
            tags: tags
                .into_iter()
                .map(|tag| tag.map(|label| label.into(), |label| label.into()))
                .collect(),
        });
        self
    }

    pub fn equality(mut self, label_f: impl Into<L>, label_g: impl Into<L>) -> Self {
        self.0
            .push(Constraint::Equality(label_f.into(), label_g.into()));
        self
    }
}
