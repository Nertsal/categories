use super::*;

impl<O, M, E> Category<O, M, E> {
    /// Applies the rule constraints to the graph.
    pub fn apply_constraints<'a, L: 'a + Label>(
        &mut self,
        constraints: impl IntoIterator<Item = &'a Constraint<L>> + 'a,
        bindings: &Bindings<L>,
        object_constructor: &impl Fn(Vec<ObjectTag<&Object<O>>>) -> O,
        morphism_constructor: &impl Fn(
            MorphismConnection<&Object<O>>,
            Vec<MorphismTag<&Object<O>, &Morphism<M>>>,
        ) -> M,
        equality_constructor: impl Fn(&Equality) -> E,
    ) -> (Vec<Action<O, M, E>>, Bindings<L>) {
        let mut bindings = bindings.clone();

        let mut constrained_objects = Vec::new();
        let mut constrained_morphisms = Vec::new();
        let mut constrained_equalities = Vec::new();

        for constraint in constraints {
            match constraint {
                Constraint::Object { label, tags } => {
                    constrained_objects.push((label, tags));
                }
                Constraint::Morphism {
                    label,
                    connection,
                    tags,
                } => {
                    constrained_morphisms.push((label, connection, tags));
                }
                Constraint::Equality(equality) => {
                    constrained_equalities.push(equality);
                }
            }
        }

        let mut new_objects = Vec::new();
        let mut new_object_names = Vec::new();
        let mut extend_objects = Vec::new();
        let mut new_morphisms = Vec::new();
        let mut extend_morphisms = Vec::new();
        let mut new_morphism_names = Vec::new();

        // Constraint vertices
        for (label, tags) in constrained_objects {
            let tags = tags
                .iter()
                .map(|tag| tag.map_borrowed(|label| bindings.get_object(label).unwrap())) // TODO: proper error handling
                .collect::<Vec<_>>();

            if let Some(object) = bindings.get_object(label) {
                extend_objects.push((object, tags));
            } else {
                let label_tags = tags
                    .iter()
                    .map(|tag| tag.map_borrowed(|id| self.objects.get(id).unwrap())) // TODO: better error handling
                    .collect();

                new_objects.push(Object {
                    inner: object_constructor(label_tags),
                    tags,
                });
                new_object_names.push(label.clone());
            }
        }

        let mut action_history = Vec::new();

        // Extend vertices
        if extend_objects.len() > 0 {
            let actions = self.action_do(Action::ExtendObjectTags(extend_objects));
            assert_eq!(actions.len(), 1);
            action_history.extend(actions);
        }

        // Create new vertices
        if new_objects.len() > 0 {
            create_vertices(
                self,
                &mut bindings,
                &mut action_history,
                new_objects,
                new_object_names,
            );
        }

        // Constraint edges
        for (label, connection, tags) in constrained_morphisms {
            let tags = tags
                .iter()
                .map(|tag| {
                    tag.map_borrowed(
                        |label| bindings.get_object(label).unwrap(), // TODO: proper error handling
                        |label| bindings.get_morphism(label).unwrap(), // TODO: proper error handling
                    )
                })
                .collect::<Vec<_>>();

            if let Some(morphism_id) = bindings.get_morphism(label) {
                extend_morphisms.push((morphism_id, tags));
            } else {
                let connection = connection.map_borrowed(|label| {
                    get_object_or_new(
                        label,
                        self,
                        &mut bindings,
                        &mut action_history,
                        object_constructor,
                    )
                });
                let label_connection = connection.map_borrowed(|id| self.objects.get(id).unwrap()); // TODO: better error handling

                let label_tags = tags
                    .iter()
                    .map(|tag| {
                        tag.map_borrowed(
                            |id| self.objects.get(id).unwrap(), // TODO: better error handling
                            |id| self.morphisms.get(id).unwrap(), // TODO: better error handling
                        )
                    })
                    .collect();

                new_morphisms.push(Morphism {
                    inner: morphism_constructor(label_connection, label_tags),
                    connection,
                    tags,
                });
                new_morphism_names.push(label.clone());
            }
        }

        // Extend edges
        if extend_morphisms.len() > 0 {
            let actions = self.action_do(Action::ExtendMorphismTags(extend_morphisms));
            assert_eq!(actions.len(), 1);
            action_history.extend(actions);
        }

        // Create new edges
        if new_morphisms.len() > 0 {
            let actions = self.action_do(Action::NewMorphisms(new_morphisms));
            assert_eq!(actions.len(), 1);
            // Bind new edges
            match &actions[0] {
                Action::RemoveMorphisms(edges) => {
                    assert_eq!(edges.len(), new_morphism_names.len());
                    for (label, id) in new_morphism_names.into_iter().zip(edges.iter().copied()) {
                        bindings.bind_morphism(label, id);
                    }
                }
                _ => unreachable!(),
            }
            action_history.extend(actions);
        }

        // Constraint equalities
        let constrained_equalities = constrained_equalities
            .into_iter()
            .filter_map(|equality| {
                let process = |composition: &Vec<L>| {
                    let mut result = Vec::with_capacity(composition.len());
                    for label in composition {
                        match bindings.get_morphism(label) {
                            None => return None,
                            Some(id) => {
                                let morphism = self.morphisms.get(&id).unwrap();
                                let decomposed = util::decompose_morphism(id, morphism, self);
                                result.extend(decomposed);
                            }
                        }
                    }
                    Some(result)
                };
                process(equality.left()).and_then(|left| {
                    process(equality.right()).map(|right| {
                        let equality =
                            Equality::new(left, right).expect("Failed to apply equality");
                        let inner = equality_constructor(&equality);
                        (equality, inner)
                    })
                })
            })
            .collect::<Vec<_>>();

        if constrained_equalities.len() > 0 {
            let actions = self.action_do(Action::NewEqualities(constrained_equalities));
            assert_eq!(actions.len(), 1);
            action_history.extend(actions);
        }

        (action_history, bindings)
    }
}

fn create_vertices<O, M, E, L: Label>(
    category: &mut Category<O, M, E>,
    bindings: &mut Bindings<L>,
    action_history: &mut Vec<Action<O, M, E>>,
    new_vertices: Vec<Object<O>>,
    new_vertices_names: Vec<L>,
) -> Vec<ObjectId> {
    let new_vertices = new_vertices
        .into_iter()
        .map(|object| (None, object))
        .collect();
    let actions = category.action_do(Action::NewObjects(new_vertices));
    assert_eq!(actions.len(), 1);
    // Bind new vertices
    let new_vertices = match &actions[0] {
        Action::RemoveObjects(vertices) => {
            assert_eq!(vertices.len(), new_vertices_names.len());
            for (label, id) in new_vertices_names.into_iter().zip(vertices.iter().copied()) {
                bindings.bind_object(label, id);
            }
            vertices.clone()
        }
        _ => unreachable!(),
    };
    action_history.extend(actions);
    new_vertices
}

fn get_object_or_new<O, M, E, L: Label>(
    label: &L,
    category: &mut Category<O, M, E>,
    bindings: &mut Bindings<L>,
    action_history: &mut Vec<Action<O, M, E>>,
    object_constructor: impl Fn(Vec<ObjectTag<&Object<O>>>) -> O,
) -> ObjectId {
    bindings.get_object(label).unwrap_or_else(|| {
        create_vertices(
            category,
            bindings,
            action_history,
            vec![Object {
                tags: vec![],
                inner: object_constructor(vec![]),
            }],
            vec![label.clone()],
        )[0]
    })
}
