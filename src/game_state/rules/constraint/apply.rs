use super::*;

/// Applies the rule constraints to the graph.
pub fn apply_constraints(
    category: &mut Category,
    equalities: &mut Equalities,
    constraints: &Constraints,
    bindings: &Bindings,
) -> (Vec<GraphAction>, Bindings) {
    let mut bindings = bindings.clone();

    let mut constrained_vertices = Vec::new();
    let mut constrained_edges = Vec::new();
    let mut constrained_equalities = Vec::new();

    for constraint in constraints {
        match constraint {
            Constraint::RuleObject(label, rule_object) => match rule_object {
                RuleObject::Object { tag } => {
                    constrained_vertices.push((label, tag));
                }
                RuleObject::Morphism { constraint } => {
                    constrained_edges.push((label, constraint));
                }
            },
            Constraint::MorphismEq(f, g) => {
                constrained_equalities.push((f, g));
            }
        }
    }

    let mut new_vertices = Vec::new();
    let mut new_vertices_names = Vec::new();
    let mut new_edges = Vec::new();
    let mut new_edges_names = Vec::new();

    // Constraint vertices
    for (label, tag) in constrained_vertices {
        let tag = tag.as_ref().map(|tag| {
            tag.map_borrowed(|label| {
                label
                    .as_ref()
                    .map(|label| bindings.get_object(label).unwrap())
            })
        });

        if let Some(_) = bindings.get_object(label) {
            // TODO: possibly need to add a tag
        } else {
            let name = tag
                .iter()
                .filter_map(|tag| {
                    tag.map_borrowed(|object| {
                        object
                            .as_ref()
                            .map(|object| &category.objects.get(object).unwrap().label)
                    })
                    .infer_name()
                })
                .find(|_| true)
                .map(|name| Label::Name(name))
                .unwrap_or(Label::Unknown);

            new_vertices.push((name, tag));
            new_vertices_names.push(label.clone());
        }
    }

    let mut action_history = Vec::new();

    // Create new vertices
    if new_vertices.len() > 0 {
        create_vertices(
            category,
            equalities,
            &mut bindings,
            &mut action_history,
            new_vertices,
            new_vertices_names,
        );
    }

    // Constraint edges
    for (label, constraint) in constrained_edges {
        let connection = match &constraint.connection {
            MorphismConnection::Regular { from, to } => MorphismConnection::Regular {
                from: get_object_or_new(
                    from,
                    category,
                    equalities,
                    &mut bindings,
                    &mut action_history,
                ),
                to: get_object_or_new(to, category, equalities, &mut bindings, &mut action_history),
            },
            MorphismConnection::Isomorphism(a, b) => MorphismConnection::Isomorphism(
                get_object_or_new(a, category, equalities, &mut bindings, &mut action_history),
                get_object_or_new(b, category, equalities, &mut bindings, &mut action_history),
            ),
        };
        let constraint = ArrowConstraint {
            connection,
            tag: constraint.tag.as_ref().map(|tag| {
                tag.map_borrowed(
                    |label| label.as_ref().and_then(|label| bindings.get_object(label)),
                    |label| {
                        label
                            .as_ref()
                            .and_then(|label| bindings.get_morphism(label))
                    },
                )
            }),
        };

        if let Some(_) = bindings.get_morphism(label) {
            // TODO: possibly add a tag
        } else {
            let name = constraint
                .tag
                .iter()
                .filter_map(|tag| {
                    tag.map_borrowed(
                        |id| {
                            id.as_ref()
                                .map(|id| &category.objects.get(id).unwrap().label)
                        },
                        |id| {
                            id.as_ref()
                                .map(|id| &category.morphisms.get(id).unwrap().inner.label)
                        },
                    )
                    .infer_name()
                })
                .find(|_| true)
                .map(|name| Label::Name(name))
                .unwrap_or(Label::Unknown);

            new_edges.push((name, constraint));
            new_edges_names.push(label.clone());
        }
    }

    // Create new edges
    if new_edges.len() > 0 {
        let actions = action::action_do(category, equalities, GraphAction::NewMorphisms(new_edges));
        assert_eq!(actions.len(), 1);
        // Bind new edges
        match &actions[0] {
            GraphAction::RemoveMorphisms(edges) => {
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

    let actions = action::action_do(
        category,
        equalities,
        GraphAction::NewEqualities(constrained_equalities),
    );
    assert_eq!(actions.len(), 1);

    action_history.extend(actions);

    (action_history, bindings)
}

fn create_vertices(
    category: &mut Category,
    equalities: &mut Equalities,
    bindings: &mut Bindings,
    action_history: &mut Vec<GraphAction>,
    new_vertices: Vec<(Label, Option<ObjectTag<Option<ObjectId>>>)>,
    new_vertices_names: Vec<Label>,
) -> Vec<ObjectId> {
    let actions = action::action_do(category, equalities, GraphAction::NewObjects(new_vertices));
    assert_eq!(actions.len(), 1);
    // Bind new vertices
    let new_vertices = match &actions[0] {
        GraphAction::RemoveObjects(vertices) => {
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

fn get_object_or_new(
    label: &Label,
    category: &mut Category,
    equalities: &mut Equalities,
    bindings: &mut Bindings,
    action_history: &mut Vec<GraphAction>,
) -> ObjectId {
    bindings.get_object(label).unwrap_or_else(|| {
        create_vertices(
            category,
            equalities,
            bindings,
            action_history,
            vec![(Label::Unknown, None)],
            vec![label.clone()],
        )[0]
    })
}
