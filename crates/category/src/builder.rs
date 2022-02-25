use std::collections::HashMap;

use super::*;

pub struct CategoryBuilder<O, M, L: Label> {
    category: Category<O, M>,
    objects: HashMap<L, ObjectId>,
    morphisms: HashMap<L, MorphismId>,
}

impl<O, M, L: Label> CategoryBuilder<O, M, L> {
    pub fn new() -> Self {
        Self {
            category: Category::new(),
            objects: HashMap::new(),
            morphisms: HashMap::new(),
        }
    }

    pub fn build(self) -> Category<O, M> {
        self.category
    }

    pub fn equality<T: Into<L>>(
        mut self,
        left: impl IntoIterator<Item = T>,
        right: impl IntoIterator<Item = T>,
    ) -> Self {
        let left = left
            .into_iter()
            .map(|label| self.morphisms[&label.into()])
            .collect();
        let right = right
            .into_iter()
            .map(|label| self.morphisms[&label.into()])
            .collect();

        self.category
            .equalities
            .new_equality(Equality::new(left, right).unwrap());

        self
    }

    pub fn object<T: Into<L>>(
        mut self,
        label: T,
        tags: impl IntoIterator<Item = ObjectTag<T>>,
        inner: O,
    ) -> Self {
        let label = label.into();
        let new_object = self.category.new_object(Object {
            tags: tags
                .into_iter()
                .map(|tag| tag.map(|label| self.objects[&label.into()]))
                .collect(),
            inner,
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
        inner: M,
    ) -> Self {
        self.some_morphism(label, MorphismConnection::Regular { from, to }, tags, inner)
    }

    pub fn isomorphism<T: Into<L>>(
        self,
        label: T,
        object_a: T,
        object_b: T,
        tags: impl IntoIterator<Item = MorphismTag<T, T>>,
        inner: M,
    ) -> Self {
        self.some_morphism(
            label,
            MorphismConnection::Isomorphism(object_a, object_b),
            tags,
            inner,
        )
    }

    fn some_morphism<T: Into<L>>(
        mut self,
        label: T,
        connection: MorphismConnection<T>,
        tags: impl IntoIterator<Item = MorphismTag<T, T>>,
        inner: M,
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

        let new_edge = self.category.new_morphism(Morphism {
            connection,
            tags,
            inner,
        });

        self.morphisms.insert(label.to_owned(), new_edge.unwrap());

        self
    }
}
