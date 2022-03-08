use std::collections::HashMap;

use super::*;

#[derive(Debug, Clone)]
pub enum Action<O, M, E> {
    NewObjects(Vec<(Option<ObjectId>, Object<O>)>),
    ExtendObjectTags(Vec<(ObjectId, Vec<ObjectTag>)>),
    RemoveObjectTags(Vec<(ObjectId, Vec<ObjectTag>)>),
    RemoveObjects(Vec<ObjectId>),
    NewMorphisms(Vec<(Option<MorphismId>, Morphism<M>)>),
    ExtendMorphismTags(Vec<(MorphismId, Vec<MorphismTag>)>),
    RemoveMorphismTags(Vec<(MorphismId, Vec<MorphismTag>)>),
    RemoveMorphisms(Vec<MorphismId>),
    NewEqualities(Vec<(Equality, E)>),
    RemoveEqualities(Vec<Equality>),
    SubstituteEqualities(Vec<(Equality, Equality)>),
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
                    .map(|(id, morphism)| {
                        match id {
                            Some(id) => {
                                let replaced = self
                                    .insert_morphism(morphism, id)
                                    .expect("Morphism ids are expected to be valid");
                                if replaced.is_some() {
                                    panic!("Cannot replace an existing morphism with another");
                                }
                                id
                            }
                            None => self
                                .new_morphism(morphism)
                                .expect("Objects are expected to exist"), // TODO: do proper handling
                        }
                    })
                    .collect();
                vec![Action::RemoveMorphisms(morphisms)]
            }
            Action::ExtendObjectTags(extensions) => {
                // Avoid duplicating tags
                let extensions = extensions
                    .into_iter()
                    .filter_map(|(id, tags)| {
                        self.objects
                            .get_mut(&id)
                            .map(|object| (object, tags))
                            .map(|(object, tags)| {
                                let tags = tags
                                    .into_iter()
                                    .filter(|tag| !object.tags.contains(tag))
                                    .collect::<Vec<_>>();
                                object.tags.extend(tags.clone());
                                (id, tags)
                            })
                            .filter(|(_, tags)| !tags.is_empty())
                    })
                    .collect::<Vec<_>>();

                if extensions.is_empty() {
                    vec![]
                } else {
                    vec![Action::RemoveObjectTags(extensions)]
                }
            }
            Action::ExtendMorphismTags(extensions) => {
                // Avoid duplicating tags
                let extensions = extensions
                    .into_iter()
                    .filter_map(|(id, tags)| {
                        self.morphisms
                            .get_mut(&id)
                            .map(|morphism| (morphism, tags))
                            .map(|(morphism, tags)| {
                                let tags = tags
                                    .into_iter()
                                    .filter(|tag| !morphism.tags.contains(tag))
                                    .collect::<Vec<_>>();
                                morphism.tags.extend(tags.clone());
                                (id, tags)
                            })
                            .filter(|(_, tags)| !tags.is_empty())
                    })
                    .collect::<Vec<_>>();

                if extensions.is_empty() {
                    vec![]
                } else {
                    vec![Action::RemoveMorphismTags(extensions)]
                }
            }
            Action::RemoveObjectTags(mut extensions) => {
                extensions.retain(|(id, _)| self.objects.contains(id));
                for (object_id, remove_tags) in &extensions {
                    let object = self.objects.get_mut(object_id).unwrap(); // Check was done when retaining
                    object.tags.retain(|tag| !remove_tags.contains(tag));
                }

                if extensions.is_empty() {
                    vec![]
                } else {
                    vec![Action::ExtendObjectTags(extensions)]
                }
            }
            Action::RemoveMorphismTags(mut extensions) => {
                extensions.retain(|(id, _)| self.morphisms.contains(id));
                for (morphism_id, remove_tags) in &extensions {
                    let morphism = self.morphisms.get_mut(morphism_id).unwrap(); // Check was done when retaining
                    morphism.tags.retain(|tag| !remove_tags.contains(tag));
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
                            .map(|(id, morphism)| (Some(id), morphism))
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
                    .filter_map(|id| {
                        self.remove_morphism(id)
                            .map(|morphism| (Some(id), morphism))
                    })
                    .collect();

                let mut undo = vec![Action::NewMorphisms(morphisms)];
                if !equalities.is_empty() {
                    undo.push(Action::NewEqualities(equalities));
                }
                undo
            }
            Action::NewEqualities(equalities) => {
                let mut actions = Vec::new();

                let mut substitutes = HashMap::new();
                // Check substitable equalities
                let equalities = equalities
                    .into_iter()
                    .filter(|(equality, _)| {
                        match (equality.left().as_slice(), equality.right().as_slice()) {
                            (&[f], &[g]) => {
                                let g = substitutes.get(&g).copied().unwrap_or(g);
                                substitutes.insert(f, g);
                                false
                            }
                            _ => true,
                        }
                    })
                    .collect::<Vec<_>>();

                // Move tags
                let extend = substitutes
                    .iter()
                    .filter_map(|(from, to)| {
                        self.morphisms
                            .get(from)
                            .map(|morphism| (*to, morphism.tags.clone()))
                    })
                    .collect();
                let extended = self.action_do(Action::ExtendMorphismTags(extend));
                actions.extend(extended);

                // Substitute into existing equalities
                let mut kept = Vec::new();
                let mut changed = Vec::new();
                for (equality, inner) in self.equalities.drain() {
                    let original = equality.clone();
                    let (mut left, mut right) = equality.destructure();
                    let mut is_changed = false;
                    for morphism in left.iter_mut().chain(&mut right) {
                        if let Some(&substitute) = substitutes.get(morphism) {
                            is_changed = true;
                            *morphism = substitute;
                        }
                    }
                    if is_changed {
                        let substituted = Equality::new(left, right).unwrap();
                        changed.push((substituted.clone(), original));
                        kept.push((substituted, inner));
                    } else {
                        kept.push((original, inner));
                    }
                }
                for (equality, inner) in kept {
                    self.equalities.new_equality(equality, inner);
                }
                actions.push(Action::SubstituteEqualities(changed));

                // Remove substituted morphisms
                let remove = substitutes.iter().map(|(from, _)| *from).collect();
                let removed = self.action_do(Action::RemoveMorphisms(remove));
                actions.extend(removed);

                // Apply substitutions
                let equalities = equalities
                    .into_iter()
                    .map(|(equality, inner)| {
                        let (mut left, mut right) = equality.destructure();
                        for morphism in left.iter_mut().chain(&mut right) {
                            *morphism = substitutes.get(morphism).copied().unwrap_or(*morphism);
                        }
                        (Equality::new(left, right).unwrap(), inner)
                    })
                    .map(|(equality, inner)| {
                        self.equalities.new_equality(equality.clone(), inner);
                        equality
                    })
                    .collect();
                actions.push(Action::RemoveEqualities(equalities));

                actions
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
            Action::SubstituteEqualities(substitutions) => {
                let substitutions = substitutions
                    .into_iter()
                    .filter_map(|(from, to)| {
                        self.equalities.remove_equality(&from).map(|inner| {
                            self.equalities.new_equality(to.clone(), inner);
                            (to, from)
                        })
                    })
                    .collect();
                vec![Action::SubstituteEqualities(substitutions)]
            }
        }
    }
}
