use super::*;

#[derive(Debug, Clone)]
pub enum GraphAction {
    ApplyRule {
        input_vertices: Vec<VertexId>,
        new_vertices: usize,
        new_edges: Vec<ArrowConstraint<usize>>,
    },
}

impl GameState {
    pub fn action_do(&mut self, action: GraphAction) {
        match action {
            GraphAction::ApplyRule {
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
                input_vertices.reserve(new_vertices);
                for _ in 0..new_vertices {
                    input_vertices.push(self.main_graph.graph.new_vertex(ForceVertex {
                        is_anchor: false,
                        body: ForceBody::new(random_shift(), POINT_MASS),
                        vertex: Point {
                            label: "".to_owned(),
                            radius: POINT_RADIUS,
                            color: Color::WHITE,
                        },
                    }))
                }

                // Add edges
                for edge in new_edges {
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
                    self.main_graph.graph.new_edge(ForceEdge::new(
                        from_pos,
                        to_pos,
                        ARROW_BODIES,
                        ARROW_MASS,
                        Arrow::new("", from, to, edge.connection, edge.connection.color()),
                    ));
                }
            }
        }
    }

    pub fn action_undo(&mut self) {
        if let Some(action) = self.action_history.pop() {
            todo!()
        }
    }
}
