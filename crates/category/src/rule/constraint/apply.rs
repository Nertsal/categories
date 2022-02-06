use super::*;

impl<O, M> Category<O, M> {
    /// Applies the rule constraints to the graph.
    pub fn apply_constraints<L: Label>(
        &mut self,
        constraints: &Constraints<L>,
        bindings: &Bindings<L>,
        object_constructor: &impl Fn(Vec<ObjectTag<&Object<O>>>) -> O,
        morphism_constructor: &impl Fn(
            MorphismConnection<&Object<O>>,
            Vec<MorphismTag<&Object<O>, &Morphism<M>>>,
        ) -> M,
    ) -> (Vec<Action<O, M>>, Bindings<L>) {
        let mut bindings = bindings.clone();

        let mut constrained_vertices = Vec::new();
        let mut constrained_edges = Vec::new();
        let mut constrained_equalities = Vec::new();
        let mut constrained_commutes = Vec::new();

        for constraint in constraints {
            match constraint {
                Constraint::Object { label, tags } => {
                    constrained_vertices.push((label, tags));
                }
                Constraint::Morphism {
                    label,
                    connection,
                    tags,
                } => {
                    constrained_edges.push((label, connection, tags));
                }
                Constraint::Equality(f, g) => {
                    constrained_equalities.push((f, g));
                }
                Constraint::Commute { f, g, h } => {
                    constrained_commutes.push((f, g, h));
                }
            }
        }

        let mut new_vertices = Vec::new();
        let mut new_vertices_names = Vec::new();
        let mut new_edges = Vec::new();
        let mut new_edges_names = Vec::new();

        // Constraint vertices
        for (label, tags) in constrained_vertices {
            let tags = tags
                .iter()
                .map(|tag| tag.map_borrowed(|label| bindings.get_object(label).unwrap())) // TODO: proper error handling
                .collect::<Vec<_>>();

            if let Some(_) = bindings.get_object(label) {
                // TODO: possibly need to add a tag
            } else {
                let label_tags = tags
                    .iter()
                    .map(|tag| tag.map_borrowed(|id| self.objects.get(id).unwrap())) // TODO: better error handling
                    .collect();

                new_vertices.push(Object {
                    inner: object_constructor(label_tags),
                    tags,
                });
                new_vertices_names.push(label.clone());
            }
        }

        let mut action_history = Vec::new();

        // Create new vertices
        if new_vertices.len() > 0 {
            create_vertices(
                self,
                &mut bindings,
                &mut action_history,
                new_vertices,
                new_vertices_names,
            );
        }

        // Constraint edges
        for (label, connection, tags) in constrained_edges {
            let connection = connection.map_borrowed(|label| {
                get_object_or_new(
                    label,
                    self,
                    &mut bindings,
                    &mut action_history,
                    object_constructor,
                )
            });

            let tags = tags
                .iter()
                .map(|tag| {
                    tag.map_borrowed(
                        |label| bindings.get_object(label).unwrap(), // TODO: proper error handling
                        |label| bindings.get_morphism(label).unwrap(), // TODO: proper error handling
                    )
                })
                .collect::<Vec<_>>();

            if let Some(_) = bindings.get_morphism(label) {
                // TODO: possibly add a tag
            } else {
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

                new_edges.push(Morphism {
                    inner: morphism_constructor(label_connection, label_tags),
                    connection,
                    tags,
                });
                new_edges_names.push(label.clone());
            }
        }

        // Create new edges
        if new_edges.len() > 0 {
            let actions = self.action_do(Action::NewMorphisms(new_edges));
            assert_eq!(actions.len(), 1);
            // Bind new edges
            match &actions[0] {
                Action::RemoveMorphisms(edges) => {
                    assert_eq!(edges.len(), new_edges_names.len());
                    for (label, id) in new_edges_names.into_iter().zip(edges.iter().copied()) {
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
            .filter_map(|(f, g)| {
                bindings
                    .get_morphism(f)
                    .and_then(|f| bindings.get_morphism(g).map(|g| (f, g)))
            })
            .collect();

        let actions = self.action_do(Action::NewEqualities(constrained_equalities));
        assert_eq!(actions.len(), 1);
        action_history.extend(actions);

        // Constraint commutativities
        let constrained_commutes = constrained_commutes
            .into_iter()
            .filter_map(|(f, g, h)| {
                bindings.get_morphism(f).and_then(|f| {
                    bindings
                        .get_morphism(g)
                        .and_then(|g| bindings.get_morphism(h).map(|h| (f, g, h)))
                })
            })
            .collect();

        let actions = self.action_do(Action::NewCommutes(constrained_commutes));
        assert_eq!(actions.len(), 1);
        action_history.extend(actions);

        (action_history, bindings)
    }
}

fn create_vertices<O, M, L: Label>(
    category: &mut Category<O, M>,
    bindings: &mut Bindings<L>,
    action_history: &mut Vec<Action<O, M>>,
    new_vertices: Vec<Object<O>>,
    new_vertices_names: Vec<L>,
) -> Vec<ObjectId> {
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

fn get_object_or_new<O, M, L: Label>(
    label: &L,
    category: &mut Category<O, M>,
    bindings: &mut Bindings<L>,
    action_history: &mut Vec<Action<O, M>>,
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