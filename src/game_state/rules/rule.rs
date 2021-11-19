use graphs::GraphEdge;

use super::*;

impl GameState {
    /// Attempts to apply a rule.
    /// Returns whether the rule was applied successfully.
    pub fn apply_rule(&mut self, selection: RuleSelection) -> bool {
        let rule = self.rules.get_rule(selection.rule()).unwrap();
        if !rule.check_input(&self.main_graph, selection.selection()) {
            return false;
        }

        // TODO: infer

        rule.apply(&mut self.main_graph, selection.to_selection());
        true
    }
}

pub struct Rule {
    inputs: Vec<RuleObject<String>>,
    infers: Vec<RuleObject<String>>,
    outputs: Vec<RuleObject<String>>,
    graph: Graph,
    graph_input: Vec<GraphObject>,
}

impl Rule {
    pub fn new<'a>(
        inputs: Vec<RuleObject<&'a str>>,
        infers: Vec<RuleObject<&'a str>>,
        outputs: Vec<RuleObject<&'a str>>,
    ) -> Self {
        // Create a graph
        let mut graph = Graph::new(ForceParameters::default());

        let mut add_object =
            |object: &RuleObject<&str>, color: Color<f32>, override_color: bool| match object {
                RuleObject::Vertex { label } => GraphObject::Vertex {
                    id: get_vertex_id(&mut graph, label, Ok(Some(color))),
                },
                RuleObject::Edge { label, constraint } => {
                    let vertex_color = if override_color { Err(color) } else { Ok(None) };
                    let from = get_vertex_id(&mut graph, &constraint.from, vertex_color);
                    let to = get_vertex_id(&mut graph, &constraint.to, vertex_color);
                    GraphObject::Edge {
                        id: graph
                            .graph
                            .new_edge(ForceEdge::new(
                                random_pos(),
                                random_pos(),
                                ARROW_BODIES,
                                ARROW_MASS,
                                Arrow::new(label, from, to, constraint.connection, color),
                            ))
                            .unwrap(),
                    }
                }
            };

        // Input
        let graph_input = inputs
            .iter()
            .map(|input| add_object(input, RULE_INPUT_COLOR, false))
            .collect();

        // Infer
        for infer in &infers {
            add_object(infer, RULE_INFER_COLOR, true);
        }

        // Output
        for output in &outputs {
            add_object(output, RULE_OUTPUT_COLOR, true);
        }

        fn convert(objects: Vec<RuleObject<&str>>) -> Vec<RuleObject<String>> {
            objects
                .into_iter()
                .map(|object| match object {
                    RuleObject::Vertex { label } => RuleObject::Vertex {
                        label: label.to_owned(),
                    },
                    RuleObject::Edge { label, constraint } => RuleObject::Edge {
                        label: label.to_owned(),
                        constraint: ArrowConstraint::new(
                            constraint.from.to_owned(),
                            constraint.to.to_owned(),
                            constraint.connection,
                        ),
                    },
                })
                .collect()
        }

        Self {
            inputs: convert(inputs),
            infers: convert(infers),
            outputs: convert(outputs),
            graph,
            graph_input,
        }
    }

    pub fn get_input(&self) -> &Vec<GraphObject> {
        &self.graph_input
    }

    fn get_vertex_by_label(graph: &Graph, label: &str) -> Option<VertexId> {
        graph
            .graph
            .vertices
            .iter()
            .find(|(_, vertex)| vertex.vertex.label.eq(label))
            .map(|(&id, _)| id)
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    pub fn graph_mut(&mut self) -> &mut Graph {
        &mut self.graph
    }

    pub fn update_graph(&mut self, delta_time: f32) {
        self.graph.update(delta_time);
    }

    /// Checks that input meets the rule's constraints.
    fn check_input(&self, graph: &Graph, selection: &Vec<GraphObject>) -> bool {
        // Check length
        if selection.len() != self.inputs.len() {
            return false;
        }

        // Check contents
        let mut vertices = HashMap::new();
        for object in self.inputs.iter().zip(selection.iter()) {
            let fit = match object {
                (RuleObject::Vertex { label }, &GraphObject::Vertex { id }) => vertices
                    .insert(label, id)
                    .map(|old_id| old_id == id)
                    .unwrap_or(true),
                (RuleObject::Edge { constraint, .. }, GraphObject::Edge { id }) => graph
                    .graph
                    .edges
                    .get(id)
                    .map(|edge| {
                        edge.edge.check_constraint(&ArrowConstraint {
                            from: *vertices.entry(&constraint.from).or_insert(edge.edge.from),
                            to: *vertices.entry(&constraint.to).or_insert(edge.edge.to),
                            connection: constraint.connection,
                        })
                    })
                    .unwrap_or_default(),
                _ => false,
            };
            if !fit {
                return false;
            }
        }

        true
    }

    /// Applies the rule
    fn apply(&self, graph: &mut Graph, selection: Vec<GraphObject>) {
        let mut vertices = HashMap::new();

        // Find input
        for input in self.inputs.iter().zip(selection.iter()) {
            let mut insert_vertex = |label: &str, id: VertexId| {
                vertices
                    .insert(label.to_owned(), id)
                    .map(|old_id| old_id == id)
                    .unwrap_or(true)
            };

            let insert = match input {
                (RuleObject::Vertex { label }, &GraphObject::Vertex { id }) => {
                    insert_vertex(label, id)
                }
                (RuleObject::Edge { constraint, .. }, &GraphObject::Edge { id }) => {
                    let edge = &graph.graph.edges.get(&id).unwrap().edge;
                    insert_vertex(&constraint.from, edge.from)
                        && insert_vertex(&constraint.to, edge.to)
                }
                _ => unreachable!("Must be an error in Rule::check_input"),
            };
            assert!(insert, "Unexpected: some selections are wrong");
        }

        /// Find or create a vertex
        fn get_vertex_id(
            graph: &mut Graph,
            label: &str,
            vertices: &mut HashMap<String, VertexId>,
        ) -> VertexId {
            *vertices.entry(label.to_owned()).or_insert_with(|| {
                graph.graph.new_vertex(ForceVertex {
                    is_anchor: false,
                    body: ForceBody::new(random_pos(), POINT_MASS),
                    vertex: Point {
                        label: "".to_owned(),
                        radius: POINT_RADIUS,
                        color: Color::WHITE,
                    },
                })
            })
        }

        // Infer
        // A collection of all vertices from the graph that satisfy the inferring constraints.
        let mut inferred_vertices = HashMap::new();
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

        for infer in &self.infers {
            match infer {
                RuleObject::Vertex { label } => {
                    infer_vertex(graph, label, &mut inferred_vertices, &vertices, None);
                }
                RuleObject::Edge { constraint, .. } => {
                    // Check from
                    let infer_to: Vec<_> = infer_vertex(
                        graph,
                        &constraint.to,
                        &mut inferred_vertices,
                        &vertices,
                        Some(constraint.clone()),
                    )
                    .iter()
                    .copied()
                    .collect();

                    let infer_from = infer_vertex(
                        graph,
                        &constraint.from,
                        &mut inferred_vertices,
                        &vertices,
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
                        &vertices,
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

        let mut new_connections = Vec::new();
        for (label, (candidates, edges)) in inferred_vertices {
            if !candidates.is_empty() {
                // Inferred a vertex -> add it to the list
                vertices.insert(label, candidates[0]);
            } else {
                new_connections.extend(edges);
            }
        }
        new_connections.sort();
        new_connections.dedup();

        for edge in new_connections {
            let from = get_vertex_id(graph, &edge.from, &mut vertices);
            let to = get_vertex_id(graph, &edge.to, &mut vertices);
            graph.graph.new_edge(ForceEdge::new(
                random_pos(),
                random_pos(),
                ARROW_BODIES,
                ARROW_MASS,
                Arrow::new("", from, to, edge.connection, edge.connection.color()),
            ));
        }

        // Result
        for output in &self.outputs {
            match output {
                RuleObject::Vertex { label } => {
                    get_vertex_id(graph, label, &mut vertices);
                }
                RuleObject::Edge { constraint, .. } => {
                    let from = get_vertex_id(graph, &constraint.from, &mut vertices);
                    let to = get_vertex_id(graph, &constraint.to, &mut vertices);
                    graph.graph.new_edge(ForceEdge::new(
                        random_pos(),
                        random_pos(),
                        ARROW_BODIES,
                        ARROW_MASS,
                        Arrow::new(
                            "",
                            from,
                            to,
                            constraint.connection,
                            constraint.connection.color(),
                        ),
                    ));
                }
            }
        }
    }
}

fn random_pos() -> Vec2<f32> {
    let mut rng = global_rng();
    vec2(rng.gen(), rng.gen())
}

/// Color:
/// Ok(None)        -> Do nothing or use default color (inferred from context)
/// Ok(Some(color)) -> Override existing color
/// Err(color)      -> Create new with the given color
fn get_vertex_id(
    graph: &mut Graph,
    label: &str,
    color: Result<Option<Color<f32>>, Color<f32>>,
) -> VertexId {
    match Rule::get_vertex_by_label(graph, label) {
        Some(id) => {
            if let Ok(Some(color)) = color {
                graph.graph.vertices.get_mut(&id).unwrap().vertex.color = color;
            }
            id
        }
        None => graph.graph.new_vertex(ForceVertex {
            is_anchor: false,
            body: ForceBody::new(random_pos(), POINT_MASS),
            vertex: Point {
                label: label.to_owned(),
                radius: POINT_RADIUS,
                color: match color {
                    Ok(color) => color.unwrap_or(RULE_INFER_CONTEXT_COLOR),
                    Err(color) => color,
                },
            },
        }),
    }
}
