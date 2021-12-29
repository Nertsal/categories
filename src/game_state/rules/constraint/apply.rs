use super::*;

/// Applies the rule constraints to the graph.
pub fn apply_constraints(
    graph: &mut Graph,
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

    for constraint in constraints {
        match constraint {
            Constraint::RuleObject(label, rule_object) => match rule_object {
                RuleObject::Vertex { tag } => {
                    constrained_vertices.push((label, tag));
                }
                RuleObject::Edge { constraint } => {
                    constrained_edges.push((label, constraint));
                }
            },
            Constraint::MorphismEq(_, _) => todo!(),
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

    // Create new vertices
    let mut action_history = Vec::new();
    if new_vertices.len() > 0 {
        let actions = GameState::graph_action_do(graph, GraphAction::NewVertices(new_vertices));
        assert_eq!(actions.len(), 1);
        // Bind new vertices
        match &actions[0] {
            GraphAction::RemoveVertices(vertices) => {
                assert_eq!(vertices.len(), new_vertices_names.len());
                for (label, id) in new_vertices_names.into_iter().zip(vertices.iter().copied()) {
                    bindings.bind_object(label, id);
                }
            }
            _ => unreachable!(),
        }
        action_history.extend(actions);
    }

    // Constraint edges
    for (label, constraint) in constrained_edges {
        let constraint = ArrowConstraint {
            from: bindings.get_object(&constraint.from).unwrap(),
            to: bindings.get_object(&constraint.to).unwrap(),
            tag: constraint.tag.as_ref().map(|tag| {
                tag.map_borrowed(
                    |label| {
                        label
                            .as_ref()
                            .map(|label| bindings.get_object(label).unwrap())
                    },
                    |label| {
                        label
                            .as_ref()
                            .map(|label| bindings.get_morphism(label).unwrap())
                    },
                )
            }),
        };
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

    // Create new edges
    if new_edges.len() > 0 {
        let actions = GameState::graph_action_do(graph, GraphAction::NewEdges(new_edges));
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

    (action_history, bindings)
}
