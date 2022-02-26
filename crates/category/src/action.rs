use super::*;

#[derive(Debug, Clone)]
pub enum Action<O, M, E> {
    NewObjects(Vec<(Option<ObjectId>, Object<O>)>),
    ExtendObjectTags(Vec<(ObjectId, Vec<ObjectTag>)>),
    RemoveObjectTags(Vec<(ObjectId, Vec<ObjectTag>)>),
    RemoveObjects(Vec<ObjectId>),
    NewMorphisms(Vec<Morphism<M>>),
    ExtendMorphismTags(Vec<(MorphismId, Vec<MorphismTag>)>),
    RemoveMorphismTags(Vec<(MorphismId, Vec<MorphismTag>)>),
    RemoveMorphisms(Vec<MorphismId>),
    NewEqualities(Vec<(Equality, E)>),
    RemoveEqualities(Vec<Equality>),
}

impl<O, M, E> Category<O, M, E> {
    /// Perform the action and returns the inverse action that can be used to undo the action.
    pub fn action_do(&mut self, action_do: Action<O, M, E>) -> Vec<Action<O, M, E>> {
        match action_do {
            Action::NewObjects(objects) => {
                let objects = objects
                    .into_iter()
                    .map(|(id, object)| match id {
                        Some(id) => {
                            let replaced = self
                                .insert_object(object, id)
                                .expect("Object ids are expected to be valid");
                            if replaced.is_some() {
                                panic!("Cannot replace an existing object with another");
                            }
                            id
                        }
                        None => self.new_object(object),
                    })
                    .collect();
                vec![Action::RemoveObjects(objects)]
            }
            Action::NewMorphisms(morphisms) => {
                let morphisms = morphisms
                    .into_iter()
                    .map(|morphism| {
                        self.new_morphism(morphism)
                            .expect("Objects are expected to exist") // TODO: do proper handling
                    })
                    .collect();
                vec![Action::RemoveMorphisms(morphisms)]
            }
            Action::ExtendObjectTags(mut extensions) => {
                extensions.retain(|(id, _)| self.objects.contains(id));
                for (object_id, new_tags) in extensions.clone() {
                    let object = self.objects.get_mut(&object_id).unwrap(); // Check was done when retaining
                    object.tags.extend(new_tags);
                }
                if extensions.is_empty() {
                    vec![]
                } else {
                    vec![Action::RemoveObjectTags(extensions)]
                }
            }
            Action::ExtendMorphismTags(mut extensions) => {
                extensions.retain(|(id, _)| self.morphisms.contains(id));
                for (morphism_id, new_tags) in extensions.clone() {
                    let morphism = self.morphisms.get_mut(&morphism_id).unwrap(); // Check was done when retaining
                    morphism.tags.extend(new_tags);
                }

                if extensions.is_empty() {
                    vec![]
                } else {
                    vec![Action::RemoveMorphismTags(extensions)]
                }
            }
            Action::RemoveObjectTags(mut extensions) => {
                extensions.retain(|(id, _)| self.objects.contains(id));
                for (object_id, new_tags) in &extensions {
                    let object = self.objects.get_mut(object_id).unwrap(); // Check was done when retaining
                    object.tags.retain(|tag| !new_tags.contains(tag));
                }

                if extensions.is_empty() {
                    vec![]
                } else {
                    vec![Action::ExtendObjectTags(extensions)]
                }
            }
            Action::RemoveMorphismTags(mut extensions) => {
                extensions.retain(|(id, _)| self.morphisms.contains(id));
                for (morphism_id, new_tags) in &extensions {
                    let morphism = self.morphisms.get_mut(morphism_id).unwrap(); // Check was done when retaining
                    morphism.tags.retain(|tag| !new_tags.contains(tag));
                }

                if extensions.is_empty() {
                    vec![]
                } else {
                    vec![Action::ExtendMorphismTags(extensions)]
                }
            }
            Action::RemoveObjects(objects) => {
                let (objects, morphisms) = objects
                    .into_iter()
                    .filter_map(|id| {
                        self.remove_object(id)
                            .map(|(object, morphisms)| (id, object, morphisms))
                    })
                    .map(|(object_id, object, morphisms)| {
                        let morphisms: Vec<_> = morphisms
                            .into_iter()
                            .map(|(_, morphism)| morphism)
                            .collect();
                        ((object_id, object), morphisms)
                    })
                    .fold(
                        (Vec::new(), Vec::new()),
                        |(mut acc_objects, mut acc_morphisms), ((object_id, object), morphisms)| {
                            acc_objects.push((Some(object_id), object));
                            acc_morphisms.extend(morphisms);
                            (acc_objects, acc_morphisms)
                        },
                    );
                let mut undo = vec![Action::NewObjects(objects)];
                if !morphisms.is_empty() {
                    undo.push(Action::NewMorphisms(morphisms));
                };
                undo
            }
            Action::RemoveMorphisms(morphisms) => {
                let equalities: Vec<_> = morphisms
                    .iter()
                    .flat_map(|&morphism| {
                        let equals: Vec<_> = self
                            .equalities
                            .get_equalities_with(morphism)
                            .cloned()
                            .collect();
                        let equals: Vec<_> = equals
                            .into_iter()
                            .map(|equality| {
                                let inner = self.equalities.remove_equality(&equality).unwrap();
                                (equality, inner)
                            })
                            .collect();
                        equals
                    })
                    .collect();
                let morphisms: Vec<_> = morphisms
                    .into_iter()
                    .filter_map(|id| self.remove_morphism(id))
                    .collect();

                let mut undo = vec![Action::NewMorphisms(morphisms)];
                if !equalities.is_empty() {
                    undo.push(Action::NewEqualities(equalities));
                }
                undo
            }
            Action::NewEqualities(equals) => {
                let equals = equals
                    .into_iter()
                    .map(|(equality, inner)| {
                        self.equalities.new_equality(equality.clone(), inner);
                        equality
                    })
                    .collect();
                vec![Action::RemoveEqualities(equals)]
            }
            Action::RemoveEqualities(equals) => {
                let equals = equals
                    .into_iter()
                    .filter_map(|equality| {
                        self.equalities
                            .remove_equality(&equality)
                            .map(|inner| (equality, inner))
                    })
                    .collect();
                vec![Action::NewEqualities(equals)]
            }
        }
    }
}
