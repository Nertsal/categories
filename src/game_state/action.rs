use super::*;

#[derive(Debug, Clone)]
pub enum GraphActionDo {
    ApplyRule {
        input_vertices: Vec<VertexId>,
        new_vertices: usize,
        new_edges: Vec<ArrowConstraint<usize>>,
    },
}

pub enum GraphActionUndo {
    ApplyRule {
        remove_vertices: Vec<VertexId>,
        remove_edges: Vec<EdgeId>,
    },
}

impl GameState {
    pub fn action_do(&mut self, action_do: GraphActionDo) {
        let action_undo = match action_do {
            GraphActionDo::ApplyRule {
                mut input_vertices,
                new_vertices,
                new_edges,
            } => {
                // Validate
                let available_vertices = input_vertices.len() + new_vertices;
                if !new_edges
                    .iter()
                    .all(|edge| edge.from < available_vertices || edge.to < available_vertices)
                {
                    warn!("Attempted to apply an illegal rule");
                    return;
                }

                // Create new vertices
                let new_len = new_vertices;
                let mut new_vertices = Vec::with_capacity(new_len);
                input_vertices.reserve(new_len);
                for _ in 0..new_len {
                    let vertex = self.main_graph.graph.new_vertex(ForceVertex {
                        is_anchor: false,
                        body: ForceBody::new(random_shift(), POINT_MASS),
                        vertex: Point {
                            label: "".to_owned(),
                            radius: POINT_RADIUS,
                            color: Color::WHITE,
                        },
                    });
                    new_vertices.push(vertex);
                    input_vertices.push(vertex);
                }

                // Add edges
                let new_edges: Vec<_> = new_edges
                    .into_iter()
                    .map(|edge| {
                        let from = input_vertices[edge.from];
                        let to = input_vertices[edge.to];
                        let from_pos = self
                            .main_graph
                            .graph
                            .vertices
                            .get(&from)
                            .unwrap()
                            .body
                            .position
                            + random_shift();
                        let to_pos = self
                            .main_graph
                            .graph
                            .vertices
                            .get(&to)
                            .unwrap()
                            .body
                            .position
                            + random_shift();
                        self.main_graph
                            .graph
                            .new_edge(ForceEdge::new(
                                from_pos,
                                to_pos,
                                ARROW_BODIES,
                                ARROW_MASS,
                                Arrow::new("", from, to, edge.connection, edge.connection.color()),
                            ))
                            .unwrap()
                    })
                    .collect();

                GraphActionUndo::ApplyRule {
                    remove_vertices: new_vertices,
                    remove_edges: new_edges,
                }
            }
        };

        self.action_history.push(action_undo);
    }

    pub fn action_undo(&mut self) {
        if let Some(action) = self.action_history.pop() {
            match action {
                GraphActionUndo::ApplyRule {
                    remove_vertices,
                    remove_edges,
                } => {
                    for vertex_id in remove_vertices {
                        self.main_graph.graph.remove_vertex(vertex_id);
                    }
                    for edge_id in remove_edges {
                        self.main_graph.graph.remove_edge(edge_id);
                    }
                }
            }
        }
    }
}
