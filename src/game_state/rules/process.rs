use graphs::GraphEdge;

use super::*;

#[derive(Default)]
pub(super) struct RuleProcess {
    pub(super) input_vertices: HashMap<String, VertexId>,
    pub(super) new_vertices: HashSet<String>,
    pub(super) new_edges: Vec<ArrowConstraint<String>>,
    pub(super) remove_vertices: Vec<String>,
    pub(super) remove_edges: Vec<ArrowConstraint<String>>,
}

impl RuleProcess {
    pub fn input<'a>(
        graph: &Graph,
        inputs: impl Iterator<Item = &'a RuleObject<String>>,
        selection: impl Iterator<Item = &'a GraphObject>,
    ) -> Self {
        let mut input_vertices = HashMap::new();

        for input in inputs.zip(selection) {
            let mut insert_vertex = |label: &str, id: VertexId| {
                input_vertices.entry(label.to_owned()).or_insert(id);
            };

            match input {
                (RuleObject::Vertex { label }, &GraphObject::Vertex { id }) => {
                    insert_vertex(label, id);
                }
                (RuleObject::Edge { constraint, .. }, &GraphObject::Edge { id }) => {
                    let edge = &graph.graph.edges.get(&id).unwrap().edge;
                    insert_vertex(&constraint.from, edge.from);
                    insert_vertex(&constraint.to, edge.to);
                }
                _ => unreachable!("Must be an error in Rule::check_input"),
            };
        }

        Self {
            input_vertices,
            ..Default::default()
        }
    }

    pub fn infer_candidates<'a>(
        &self,
        graph: &Graph,
        infers: impl Iterator<Item = &'a RuleObject<String>>,
    ) -> HashMap<String, (Vec<VertexId>, Vec<ArrowConstraint<String>>)> {
        // A collection of all vertices from the graph that satisfy the inferring constraints.
        let mut inferred_vertices = HashMap::new();

        /// Get information about the vertex
        fn infer_vertex<'a>(
            graph: &'a Graph,
            label: &str,
            inferred_vertices: &'a mut HashMap<
                String,
                (Vec<VertexId>, Vec<ArrowConstraint<String>>),
            >,
            rule_vertices: &HashMap<String, VertexId>,
            connection: Option<ArrowConstraint<String>>,
        ) -> &'a mut Vec<VertexId> {
            let (vertices, edges) =
                inferred_vertices
                    .entry(label.to_owned())
                    .or_insert_with(|| {
                        (
                            rule_vertices
                                .get(label)
                                .map(|&id| vec![id])
                                .unwrap_or_else(|| {
                                    graph.graph.vertices.iter().map(|(&id, _)| id).collect()
                                }),
                            vec![],
                        )
                    });
            if let Some(connection) = connection {
                edges.push(connection);
            }
            vertices
        }

        // Filter vertices
        for infer in infers {
            match infer {
                RuleObject::Vertex { label } => {
                    infer_vertex(
                        graph,
                        label,
                        &mut inferred_vertices,
                        &self.input_vertices,
                        None,
                    );
                }
                RuleObject::Edge { constraint, .. } => {
                    // Check from
                    let infer_to: Vec<_> = infer_vertex(
                        graph,
                        &constraint.to,
                        &mut inferred_vertices,
                        &self.input_vertices,
                        Some(constraint.clone()),
                    )
                    .iter()
                    .copied()
                    .collect();

                    let infer_from = infer_vertex(
                        graph,
                        &constraint.from,
                        &mut inferred_vertices,
                        &self.input_vertices,
                        Some(constraint.clone()),
                    );

                    infer_from.retain(|from| {
                        graph.graph.edges.iter().any(|(_, edge)| {
                            let end_points = edge.end_points();
                            edge.edge
                                .connection
                                .check_constraint(&constraint.connection)
                                && (end_points[0].eq(from) && infer_to.contains(end_points[1]))
                        })
                    });

                    // Check to
                    let infer_from: Vec<_> = infer_from.iter().copied().collect();

                    let infer_to = infer_vertex(
                        graph,
                        &constraint.to,
                        &mut inferred_vertices,
                        &self.input_vertices,
                        None,
                    );

                    infer_to.retain(|to| {
                        graph.graph.edges.iter().any(|(_, edge)| {
                            let end_points = edge.end_points();
                            edge.edge
                                .connection
                                .check_constraint(&constraint.connection)
                                && (end_points[1].eq(to) && infer_from.contains(end_points[0]))
                        })
                    });
                }
            }
        }

        inferred_vertices
    }

    pub fn constraint<'a>(
        mut self,
        graph: &Graph,
        constraints: impl Iterator<Item = &'a RuleObject<String>>,
    ) -> Result<Self, ()> {
        let inferred_candidates = self.infer_candidates(graph, constraints);
        for (label, (candidates, _)) in inferred_candidates {
            if candidates.is_empty() {
                return Err(());
            }

            self.input_vertices.entry(label).or_insert(candidates[0]);
        }

        Ok(self)
    }

    pub fn infer<'a>(
        mut self,
        graph: &Graph,
        infers: impl Iterator<Item = &'a RuleObject<String>>,
    ) -> Self {
        let mut new_connections = Vec::new();

        let inferred_candidates = self.infer_candidates(graph, infers);

        for (label, (candidates, edges)) in inferred_candidates {
            if !candidates.is_empty() {
                // Inferred a vertex -> add it to the list
                self.input_vertices.entry(label).or_insert(candidates[0]);
            } else {
                // New vertex, new edges
                new_connections.extend(edges);
            }
        }

        let mut new_vertices_map = HashMap::new();
        let mut get_vertex = |label: &str| {
            if !self.input_vertices.contains_key(label) {
                let len = new_vertices_map.len();
                new_vertices_map.insert(label.to_owned(), len + self.input_vertices.len());
            }
            label.to_owned()
        };

        let mut new_edges: Vec<_> = new_connections
            .into_iter()
            .map(|edge| {
                let from = get_vertex(&edge.from);
                let to = get_vertex(&edge.to);
                ArrowConstraint::new(from, to, edge.connection)
            })
            .collect();

        new_edges.sort();
        new_edges.dedup();

        self.new_edges = new_edges;
        self
    }

    pub fn remove<'a>(mut self, removes: impl Iterator<Item = &'a RuleObject<String>>) -> Self {
        for remove in removes {
            match remove {
                RuleObject::Vertex { label } => {
                    self.new_vertices.remove(label);

                    self.remove_vertices.push(label.to_owned());
                }
                RuleObject::Edge { constraint, .. } => {
                    if let Some(index) = self
                        .new_edges
                        .iter()
                        .enumerate()
                        .find(|(_, edge)| constraint.eq(edge))
                        .map(|(index, _)| index)
                    {
                        self.new_edges.swap_remove(index);
                    }
                    self.new_vertices.remove(&constraint.from);
                    self.new_vertices.remove(&constraint.to);

                    self.remove_edges.push(constraint.clone());
                }
            }
        }

        self
    }

    pub fn output<'a>(mut self, outputs: impl Iterator<Item = &'a RuleObject<String>>) -> Self {
        for output in outputs {
            match output {
                RuleObject::Vertex { label } => {
                    self.new_vertices.insert(label.to_owned());
                }
                RuleObject::Edge { constraint, .. } => {
                    let mut get_vertex = |label: &str| {
                        if !self.input_vertices.contains_key(label) {
                            self.new_vertices.insert(label.to_owned());
                        }
                        label.to_owned()
                    };

                    let from = get_vertex(&constraint.from);
                    let to = get_vertex(&constraint.to);
                    self.new_edges
                        .push(ArrowConstraint::new(from, to, constraint.connection));
                }
            }
        }

        self
    }

    pub fn action(self, graph: &Graph) -> Result<GraphActionDo, ()> {
        let input_vertices: Vec<_> = self.input_vertices.values().copied().collect();
        if input_vertices.len() == 0 {
            return Err(());
        }

        let new_vertices = self.new_vertices.len();

        let mut remove_edges = Vec::new();
        for constraint in self.remove_edges {
            let from = match self.input_vertices.get(&constraint.from) {
                None => return Err(()),
                Some(&id) => id,
            };
            let to = match self.input_vertices.get(&constraint.to) {
                None => return Err(()),
                Some(&id) => id,
            };
            let constraint = ArrowConstraint::new(from, to, constraint.connection);

            let edges = graph
                .graph
                .edges
                .iter()
                .filter(move |(_, edge)| edge.edge.check_constraint(&constraint))
                .map(|(&id, _)| id);

            remove_edges.extend(edges);
        }

        let remove_vertices: Vec<_> = self
            .remove_vertices
            .into_iter()
            .map(|label| self.input_vertices[&label])
            .collect();

        let vertices: HashMap<String, usize> = self
            .input_vertices
            .into_keys()
            .chain(self.new_vertices.into_iter())
            .enumerate()
            .map(|(index, name)| (name, index))
            .collect();

        let new_edges: Vec<_> = self
            .new_edges
            .into_iter()
            .map(|edge| {
                let from = vertices[&edge.from];
                let to = vertices[&edge.to];
                ArrowConstraint::new(from, to, edge.connection)
            })
            .collect();

        Ok(GraphActionDo::ApplyRule {
            input_vertices,
            new_vertices,
            new_edges,
            remove_vertices,
            remove_edges,
        })
    }
}
