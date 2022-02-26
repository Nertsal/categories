use super::*;

impl<O, M, E> Category<O, M, E> {
    pub fn to_constraints(&self) -> Constraints<CategoryThing> {
        let get_object_label = |id: ObjectId| CategoryThing::Object { id };
        let get_morphism_label = |id: MorphismId| CategoryThing::Morphism { id };

        self.objects
            .iter()
            .map(|(&id, object)| Constraint::Object {
                label: get_object_label(id),
                tags: object
                    .tags
                    .iter()
                    .map(|tag| tag.map_borrowed(|&object| get_object_label(object)))
                    .collect(),
            })
            .chain(self.morphisms.iter().map(|(&id, morphism)| {
                let connection = match morphism.connection {
                    MorphismConnection::Regular { from, to } => MorphismConnection::Regular {
                        from: get_object_label(from),
                        to: get_object_label(to),
                    },
                    MorphismConnection::Isomorphism(a, b) => {
                        MorphismConnection::Isomorphism(get_object_label(a), get_object_label(b))
                    }
                };

                Constraint::Morphism {
                    label: get_morphism_label(id),
                    connection,
                    tags: morphism
                        .tags
                        .iter()
                        .map(|tag| {
                            tag.map_borrowed(
                                |&object| get_object_label(object),
                                |&edge| get_morphism_label(edge),
                            )
                        })
                        .collect(),
                }
            }))
            .collect()
    }
}
