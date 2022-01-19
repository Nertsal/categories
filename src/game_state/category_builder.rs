use super::*;

pub struct CategoryBuilder {
    category: Category,
    objects: HashMap<String, ObjectId>,
    morphisms: HashMap<String, MorphismId>,
}

impl CategoryBuilder {
    pub fn new() -> Self {
        Self {
            category: Category::new(),
            objects: HashMap::new(),
            morphisms: HashMap::new(),
        }
    }

    pub fn build(self) -> Category {
        self.category
    }

    pub fn object(
        mut self,
        label: impl Into<Label>,
        tag: Option<ObjectTag<Label>>,
        color: Color<f32>,
        anchor: bool,
    ) -> Self {
        let label = label.into();
        let new_object = self.category.new_object(Object {
            is_anchor: anchor,
            position: util::random_shift(),
            radius: POINT_RADIUS,
            tag: tag.map(|tag| {
                tag.map(|label| match label {
                    Label::Name(label) => Some(self.objects[&label]),
                    Label::Unknown => None,
                })
            }),
            label: label.clone(),
            color,
        });

        match label {
            Label::Name(label) => {
                self.objects.insert(label, new_object);
            }
            Label::Unknown => (),
        }

        self
    }

    pub fn morphism(
        self,
        label: impl Into<Label>,
        from: &str,
        to: &str,
        tag: Option<MorphismTag<Label, Label>>,
    ) -> Self {
        self.some_morphism(label, MorphismConnection::Regular { from, to }, tag)
    }

    pub fn isomorphism(
        self,
        label: impl Into<Label>,
        object_a: &str,
        object_b: &str,
        tag: Option<MorphismTag<Label, Label>>,
    ) -> Self {
        self.some_morphism(
            label,
            MorphismConnection::Isomorphism(object_a, object_b),
            tag,
        )
    }

    fn some_morphism(
        mut self,
        label: impl Into<Label>,
        connection: MorphismConnection<&str>,
        tag: Option<MorphismTag<Label, Label>>,
    ) -> Self {
        let label = label.into();
        let color = draw::category::morphism_color(&tag);
        let connection = match connection {
            MorphismConnection::Regular { from, to } => MorphismConnection::Regular {
                from: self.objects[from],
                to: self.objects[to],
            },
            MorphismConnection::Isomorphism(a, b) => {
                MorphismConnection::Isomorphism(self.objects[a], self.objects[b])
            }
        };
        let tag = tag.map(|tag| {
            tag.map(
                |label| match label {
                    Label::Name(label) => Some(self.objects[&label]),
                    Label::Unknown => None,
                },
                |label| match label {
                    Label::Name(label) => Some(self.morphisms[&label]),
                    Label::Unknown => None,
                },
            )
        });

        let new_edge = self.category.new_morphism(Morphism {
            connection,
            inner: Arrow::new(
                label.clone(),
                tag,
                color,
                util::random_shift(),
                util::random_shift(),
            ),
        });

        match label {
            Label::Name(label) => {
                self.morphisms.insert(label.to_owned(), new_edge.unwrap());
            }
            Label::Unknown => (),
        }

        self
    }
}
