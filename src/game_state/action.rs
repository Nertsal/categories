use super::*;

#[derive(Debug, Clone)]
pub enum GraphAction {
    NewVertices(Vec<(Option<Label>, Vec<ObjectTag<VertexId>>)>),
    NewEdges(Vec<(Option<Label>, ArrowConstraint<VertexId, EdgeId>)>),
    RemoveVertices(Vec<VertexId>),
    RemoveEdges(Vec<EdgeId>),
}

impl GameState {
    /// Perform the action and returns the inverse action
    pub fn graph_action_do(graph: &mut Graph, action_do: GraphAction) -> GraphAction {
        match action_do {
            GraphAction::NewVertices(vertices) => {
                let vertices = vertices
                    .into_iter()
                    .map(|(label, tags)| {
                        let id = graph.graph.new_vertex(ForceVertex {
                            is_anchor: false,
                            body: ForceBody::new(random_shift(), POINT_MASS),
                            vertex: Point {
                                label: label.unwrap_or_default(),
                                radius: POINT_RADIUS,
                                color: Color::WHITE,
                                tags,
                            },
                        });
                        id
                    })
                    .collect();
                GraphAction::RemoveVertices(vertices)
            }
            GraphAction::NewEdges(edges) => {
                let edges = edges
                    .into_iter()
                    .map(|(label, constraint)| {
                        let from = constraint.from;
                        let to = constraint.to;
                        let from_pos =
                            graph.graph.vertices.get(&from).unwrap().body.position + random_shift();
                        let to_pos =
                            graph.graph.vertices.get(&to).unwrap().body.position + random_shift();
                        let tags = constraint.tags;
                        let color = draw::graph::morphism_color(&tags);
                        let id = graph
                            .graph
                            .new_edge(ForceEdge::new(
                                from_pos,
                                to_pos,
                                ARROW_BODIES,
                                ARROW_MASS,
                                Arrow::new(&label.unwrap_or_default(), from, to, tags, color),
                            ))
                            .unwrap();
                        id
                    })
                    .collect();
                GraphAction::RemoveEdges(edges)
            }
            GraphAction::RemoveVertices(vertices) => {
                todo!()
            }
            GraphAction::RemoveEdges(edges) => {
                todo!()
            }
        }
    }

    pub fn action_undo(&mut self) {
        if let Some(action) = self.action_history.pop() {
            Self::graph_action_do(&mut self.main_graph, action);
        }
    }
}
