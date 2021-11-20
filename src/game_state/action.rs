use super::*;

#[derive(Debug, Clone)]
pub enum GraphActionDo {
    ApplyRule {
        input_vertices: Vec<VertexId>,
        new_vertices: usize,
        new_edges: Vec<ArrowConstraint<usize>>,
        remove_vertices: Vec<VertexId>,
        remove_edges: Vec<EdgeId>,
    },
}

pub enum GraphActionUndo {
    ApplyRule {
        new_vertices: Vec<(VertexId, Vertex)>,
        new_edges: Vec<(EdgeId, Edge)>,
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
                remove_vertices,
                remove_edges,
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

                // Remove edges
                let mut removed_edges: Vec<_> = remove_edges
                    .into_iter()
                    .map(|edge_id| (edge_id, self.main_graph.graph.remove_edge(edge_id).unwrap()))
                    .collect();

                // Remove vertices
                let mut removed_vertices = Vec::new();
                for (vertex_id, vertex, edges) in remove_vertices.into_iter().map(|vertex_id| {
                    let (vertex, edges) = self.main_graph.graph.remove_vertex(vertex_id);
                    (vertex_id, vertex.unwrap(), edges)
                }) {
                    removed_vertices.push((vertex_id, vertex));
                    removed_edges.extend(edges.into_iter());
                }

                // Undo
                GraphActionUndo::ApplyRule {
                    new_vertices: removed_vertices,
                    new_edges: removed_edges,
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
                    new_vertices,
                    new_edges,
                    remove_vertices,
                    remove_edges,
                } => {
                    // Remove edges
                    for edge in remove_edges {
                        self.main_graph.graph.remove_edge(edge);
                    }

                    // Remove vertices
                    for vertex in remove_vertices {
                        self.main_graph.graph.remove_vertex(vertex);
                    }

                    // Add vertices
                    for (id, vertex) in new_vertices {
                        self.main_graph.graph.insert_vertex(vertex, id).unwrap();
                    }

                    // Add edges
                    for (id, edge) in new_edges {
                        self.main_graph.graph.insert_edge(edge, id).unwrap();
                    }
                }
            }
        }
    }
}
