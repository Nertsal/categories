use super::*;

impl GameState {
    /// Checks whether the goal has been reached
    pub fn check_goal(&mut self) {
        let bindings = self.graph_link.bindings();
        let constraints = graph_to_constraints(&self.goal_graph.graph);

        if find_candidates(&constraints, bindings, &self.main_graph.graph)
            .and_then(|mut candidates| candidates.next())
            .is_some()
        {
            //  The goal has been reached
            println!("Hooray! Goal reached!");
        }
    }
}

fn graph_to_constraints(graph: &Graph) -> Constraints {
    let get_object_label = |id: VertexId| {
        graph
            .graph
            .vertices
            .get(&id)
            .map(|vertex| &vertex.vertex.label)
            .unwrap()
            .clone()
    };

    let get_morphism_label = |id: EdgeId| {
        graph
            .graph
            .edges
            .get(&id)
            .map(|edge| &edge.edge.label)
            .unwrap()
            .clone()
    };

    graph
        .graph
        .vertices
        .iter()
        .map(|(_, vertex)| {
            Constraint::RuleObject(
                vertex.vertex.label.clone(),
                RuleObject::Vertex {
                    tags: vertex
                        .vertex
                        .tags
                        .iter()
                        .map(|tag| {
                            tag.map_borrowed(|object| object.map(|object| get_object_label(object)))
                        })
                        .collect(),
                },
            )
        })
        .chain(graph.graph.edges.iter().map(|(_, edge)| {
            Constraint::RuleObject(
                edge.edge.label.clone(),
                RuleObject::Edge {
                    constraint: ArrowConstraint::new(
                        get_object_label(edge.edge.from),
                        get_object_label(edge.edge.to),
                        edge.edge
                            .tags
                            .iter()
                            .map(|tag| {
                                tag.map_borrowed(
                                    |object| object.map(|object| get_object_label(object)),
                                    |edge| edge.map(|edge| get_morphism_label(edge)),
                                )
                            })
                            .collect(),
                    ),
                },
            )
        }))
        .collect()
}
