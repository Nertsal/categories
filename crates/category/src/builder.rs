use std::collections::HashMap;

use super::*;

pub struct CategoryBuilder<L: Label> {
    category: Category,
    objects: HashMap<L, ObjectId>,
    morphisms: HashMap<L, MorphismId>,
}

impl<L: Label> CategoryBuilder<L> {
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

    pub fn object<T: Into<L>>(
        mut self,
        label: T,
        tags: impl IntoIterator<Item = ObjectTag<T>>,
    ) -> Self {
        let label = label.into();
        let new_object = self.category.new_object(Object {
            tags: tags
                .into_iter()
                .map(|tag| tag.map(|label| self.objects[&label.into()]))
                .collect(),
        });

        self.objects.insert(label, new_object);

        self
    }

    pub fn morphism<T: Into<L>>(
        self,
        label: T,
        from: T,
        to: T,
        tags: impl IntoIterator<Item = MorphismTag<T, T>>,
    ) -> Self {
        self.some_morphism(label, MorphismConnection::Regular { from, to }, tags)
    }

    pub fn isomorphism<T: Into<L>>(
        self,
        label: T,
        object_a: T,
        object_b: T,
        tags: impl IntoIterator<Item = MorphismTag<T, T>>,
    ) -> Self {
        self.some_morphism(
            label,
            MorphismConnection::Isomorphism(object_a, object_b),
            tags,
        )
    }

    fn some_morphism<T: Into<L>>(
        mut self,
        label: T,
        connection: MorphismConnection<T>,
        tags: impl IntoIterator<Item = MorphismTag<T, T>>,
    ) -> Self {
        let label = label.into();
        let connection = match connection {
            MorphismConnection::Regular { from, to } => MorphismConnection::Regular {
                from: self.objects[&from.into()],
                to: self.objects[&to.into()],
            },
            MorphismConnection::Isomorphism(a, b) => {
                MorphismConnection::Isomorphism(self.objects[&a.into()], self.objects[&b.into()])
            }
        };
        let tags = tags
            .into_iter()
            .map(|tag| {
                tag.map(
                    |label| self.objects[&label.into()],
                    |label| self.morphisms[&label.into()],
                )
            })
            .collect();

        let new_edge = self.category.new_morphism(Morphism { connection, tags });

        self.morphisms.insert(label.to_owned(), new_edge.unwrap());

        self
    }
}
