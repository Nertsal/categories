use super::*;

#[derive(Debug, Clone)]
pub enum Action<O, M> {
    NewObjects(Vec<(Option<ObjectId>, Object<O>)>),
    ExtendObjectTags(Vec<(ObjectId, Vec<ObjectTag>)>),
    RemoveObjectTags(Vec<(ObjectId, Vec<ObjectTag>)>),
    RemoveObjects(Vec<ObjectId>),
    NewMorphisms(Vec<Morphism<M>>),
    ExtendMorphismTags(Vec<(MorphismId, Vec<MorphismTag>)>),
    RemoveMorphismTags(Vec<(MorphismId, Vec<MorphismTag>)>),
    RemoveMorphisms(Vec<MorphismId>),
    NewEqualities(Vec<Equality>),
    RemoveEqualities(Vec<Equality>),
}

impl<O, M> Category<O, M> {
    /// Perform the action and returns the inverse action that can be used to undo the action.
    pub fn action_do(&mut self, action_do: Action<O, M>) -> Vec<Action<O, M>> {
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
                        equals.iter().for_each(|equality| {
                            self.equalities.remove_equality(equality);
                        });
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
                equals.iter().cloned().for_each(|equality| {
                    self.equalities.new_equality(equality);
                });
                vec![Action::RemoveEqualities(equals)]
            }
            Action::RemoveEqualities(equals) => {
                equals.iter().for_each(|equality| {
                    self.equalities.remove_equality(equality);
                });
                vec![Action::NewEqualities(equals)]
            }
        }
    }
}
