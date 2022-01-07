use super::*;

/// Applies the rule constraints to the graph.
pub fn apply_constraints(
    graph: &mut Graph,
    graph_equalities: &mut GraphEqualities,
    constraints: &Constraints,
    bindings: &Bindings,
) -> (Vec<GraphAction>, Bindings) {
    let mut bindings = bindings.clone();

    let mut new_vertices = Vec::new();
    let mut new_vertices_names = Vec::new();
    let mut new_edges = Vec::new();
    let mut new_edges_names = Vec::new();

    let mut constrained_vertices = Vec::new();
    let mut constrained_edges = Vec::new();
    let mut constrained_equalities = Vec::new();

    for constraint in constraints {
        match constraint {
            Constraint::RuleObject(label, rule_object) => match rule_object {
                RuleObject::Vertex { tag } => {
                    constrained_vertices.push((label, tag));
                }
                RuleObject::Edge { constraint } => {
                    constrained_edges.push((label, constraint));

                    // Check that the objects exist, or create them later
                    let mut objects = vec![&constraint.from, &constraint.to];
                    objects.extend(
                        constraint
                            .tag
                            .iter()
                            .flat_map(|tag| tag.objects().into_iter().filter_map(|x| x.as_ref())),
                    );
                    for object in objects {
                        if let Label::Name(name) = object {
                            if !constrained_vertices.iter().any(|(label, _)| match label {
                                Label::Name(label) if *label == *name => true,
                                _ => false,
                            }) {
                                constrained_vertices.push((object, &None));
                            }
                        }
                    }
                }
            },
            Constraint::MorphismEq(f, g) => {
                // Check that morphisms exist
                if vec![f, g]
                    .into_iter()
                    .filter_map(|label| match label {
                        Label::Name(name) => Some(name),
                        Label::Any => None,
                    })
                    .all(|name| {
                        constrained_edges.iter().any(|(label, _)| match label {
                            Label::Name(label) if *label == *name => true,
                            _ => false,
                        })
                    })
                {
                    constrained_equalities.push((f, g));
                }
            }
        }
    }

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
                            .map(|object| &graph.graph.vertices.get(object).unwrap().vertex.label)
                    })
                    .infer_name()
                })
                .find(|_| true)
                .map(|name| Label::Name(name))
                .unwrap_or(Label::Any);

            new_vertices.push((name, tag));
            new_vertices_names.push(label.to_owned());
        }
    }

    let mut action_history = Vec::new();

    // Create new vertices
    if new_vertices.len() > 0 {
        create_vertices(
            graph,
            graph_equalities,
            &mut bindings,
            &mut action_history,
            new_vertices,
            new_vertices_names,
        );
    }

    // Constraint edges
    for (label, constraint) in constrained_edges {
        let constraint = ArrowConstraint {
            from: get_object_or_new(
                &constraint.from,
                graph,
                graph_equalities,
                &mut bindings,
                &mut action_history,
            ),
            to: get_object_or_new(
                &constraint.to,
                graph,
                graph_equalities,
                &mut bindings,
                &mut action_history,
            ),
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
                                .map(|id| &graph.graph.vertices.get(id).unwrap().vertex.label)
                        },
                        |id| {
                            id.as_ref()
                                .map(|id| &graph.graph.edges.get(id).unwrap().edge.label)
                        },
                    )
                    .infer_name()
                })
                .find(|_| true)
                .map(|name| Label::Name(name))
                .unwrap_or(Label::Any);

            new_edges.push((name, constraint));
            new_edges_names.push(label.to_owned());
        }
    }

    // Create new edges
    if new_edges.len() > 0 {
        let actions =
            GameState::graph_action_do(graph, graph_equalities, GraphAction::NewEdges(new_edges));
        assert_eq!(actions.len(), 1);
        // Bind new edges
        match &actions[0] {
            GraphAction::RemoveEdges(edges) => {
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
    for (f, g) in constrained_equalities {
        let f = bindings
            .get_morphism(f)
            .expect("Should have been constrained earlier");
        let g = bindings
            .get_morphism(g)
            .expect("Should have been constrained earlier");
        let actions = GameState::graph_action_do(
            graph,
            graph_equalities,
            GraphAction::NewEqualities(vec![(f, g)]),
        );
        assert_eq!(actions.len(), 1);

        action_history.extend(actions);
    }

    (action_history, bindings)
}

fn create_vertices(
    graph: &mut Graph,
    graph_equalities: &mut GraphEqualities,
    bindings: &mut Bindings,
    action_history: &mut Vec<GraphAction>,
    new_vertices: Vec<(Label, Option<ObjectTag<Option<VertexId>>>)>,
    new_vertices_names: Vec<Label>,
) -> Vec<VertexId> {
    let actions = GameState::graph_action_do(
        graph,
        graph_equalities,
        GraphAction::NewVertices(new_vertices),
    );
    assert_eq!(actions.len(), 1);
    // Bind new vertices
    let new_vertices = match &actions[0] {
        GraphAction::RemoveVertices(vertices) => {
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
    graph: &mut Graph,
    graph_equalities: &mut GraphEqualities,
    bindings: &mut Bindings,
    action_history: &mut Vec<GraphAction>,
) -> VertexId {
    bindings.get_object(label).unwrap_or_else(|| {
        create_vertices(
            graph,
            graph_equalities,
            bindings,
            action_history,
            vec![(Label::Any, None)],
            vec![label.clone()],
        )[0]
    })
}
