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

    pub fn object<T: Into<L>>(
        mut self,
        label: T,
        tags: impl IntoIterator<Item = ObjectTag<T>>,
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

    pub fn morphism<T: Into<L>>(
        mut self,
        label: T,
        from: T,
        to: T,
        tags: impl IntoIterator<Item = MorphismTag<T, T>>,
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

    pub fn isomorphism<T: Into<L>>(
        mut self,
        label: T,
        object_a: T,
        object_b: T,
        tags: impl IntoIterator<Item = MorphismTag<T, T>>,
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

    pub fn commutes(
        mut self,
        label_f: impl Into<L>,
        label_g: impl Into<L>,
        label_h: impl Into<L>,
    ) -> Self {
        self.0.push(Constraint::Commute {
            f: label_f.into(),
            g: label_g.into(),
            h: label_h.into(),
        });
        self
    }
}
