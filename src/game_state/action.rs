use super::*;

#[derive(Debug, Clone)]
pub enum GraphAction {
    NewVertices(Vec<(RuleLabel, Vec<ObjectTag<Option<VertexId>>>)>),
    NewEdges(Vec<(RuleLabel, ArrowConstraint<VertexId, EdgeId>)>),
    RemoveVertices(Vec<VertexId>),
    RemoveEdges(Vec<EdgeId>),
}

impl GameState {
    /// Perform the action and returns the inverse action
    pub fn graph_action_do(graph: &mut Graph, action_do: GraphAction) -> Vec<GraphAction> {
        match action_do {
            GraphAction::NewVertices(vertices) => {
                let vertices = vertices
                    .into_iter()
                    .map(|(label, tags)| {
                        let id = graph.graph.new_vertex(ForceVertex {
                            is_anchor: false,
                            body: ForceBody::new(util::random_shift(), POINT_MASS),
                            vertex: Point {
                                label,
                                radius: POINT_RADIUS,
                                color: Color::WHITE,
                                tags,
                            },
                        });
                        id
                    })
                    .collect();
                vec![GraphAction::RemoveVertices(vertices)]
            }
            GraphAction::NewEdges(edges) => {
                let edges = edges
                    .into_iter()
                    .map(|(label, constraint)| {
                        let from = constraint.from;
                        let to = constraint.to;
                        let from_pos = graph.graph.vertices.get(&from).unwrap().body.position
                            + util::random_shift();
                        let to_pos = graph.graph.vertices.get(&to).unwrap().body.position
                            + util::random_shift();
                        let tags = constraint.tags;
                        let color = draw::graph::morphism_color(&tags);
                        let id = graph
                            .graph
                            .new_edge(ForceEdge::new(
                                from_pos,
                                to_pos,
                                ARROW_BODIES,
                                ARROW_MASS,
                                Arrow {
                                    label,
                                    from,
                                    to,
                                    tags,
                                    color,
                                },
                            ))
                            .unwrap();
                        id
                    })
                    .collect();
                vec![GraphAction::RemoveEdges(edges)]
            }
            GraphAction::RemoveVertices(vertices) => {
                let (vertices, edges) = vertices
                    .into_iter()
                    .map(|id| graph.graph.remove_vertex(id))
                    .map(|(vertex, edges)| (vertex.unwrap(), edges))
                    .map(|(vertex, edges)| {
                        let vertex = (vertex.vertex.label, vertex.vertex.tags);
                        let edges: Vec<_> = edges
                            .into_iter()
                            .map(|(_, edge)| {
                                (
                                    edge.edge.label,
                                    ArrowConstraint {
                                        from: edge.edge.from,
                                        to: edge.edge.to,
                                        tags: edge.edge.tags,
                                    },
                                )
                            })
                            .collect();
                        (vertex, edges)
                    })
                    .fold(
                        (Vec::new(), Vec::new()),
                        |(mut acc_vertices, mut acc_edges), (vertex, edges)| {
                            acc_vertices.push(vertex);
                            acc_edges.extend(edges);
                            (acc_vertices, acc_edges)
                        },
                    );
                vec![
                    GraphAction::NewEdges(edges),
                    GraphAction::NewVertices(vertices),
                ]
            }
            GraphAction::RemoveEdges(edges) => {
                let edges: Vec<_> = edges
                    .into_iter()
                    .map(|id| graph.graph.remove_edge(id).unwrap())
                    .map(|edge| {
                        (
                            edge.edge.label,
                            ArrowConstraint {
                                from: edge.edge.from,
                                to: edge.edge.to,
                                tags: edge.edge.tags,
                            },
                        )
                    })
                    .collect();
                vec![GraphAction::NewEdges(edges)]
            }
        }
    }

    pub fn action_undo(&mut self) {
        if let Some(action) = self.action_history.pop() {
            Self::graph_action_do(&mut self.main_graph.graph, action);
        }
    }
}
