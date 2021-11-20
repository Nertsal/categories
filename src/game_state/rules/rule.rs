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

        let action = rule.action(&mut self.main_graph, selection.selection());
        self.action_do(action);
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
                                random_shift(),
                                random_shift(),
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

    pub fn inputs(&self) -> &Vec<RuleObject<String>> {
        &self.inputs
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
    fn action(&self, graph: &Graph, selection: &Vec<GraphObject>) -> GraphAction {
        // Find input
        // let (mut input_vertices, mut input_map) =
        //     rule_input(graph, self.inputs.iter(), selection.iter());

        // Infer
        // let (mut new_vertices_map, mut new_edges) = rule_infer(
        //     graph,
        //     self.infers.iter(),
        //     &mut input_vertices,
        //     &mut input_map,
        // );
        // let (input_vertices, input_map) = (input_vertices, input_map);

        // Output
        // rule_output(
        //     self.outputs.iter(),
        //     &input_map,
        //     &mut new_vertices_map,
        //     &mut new_edges,
        // );

        // GraphAction::ApplyRule {
        //     input_vertices,
        //     new_vertices: new_vertices_map.len(),
        //     new_edges,
        // }

        RuleProcess::input(graph, self.inputs.iter(), selection.iter())
            .infer(graph, self.infers.iter())
            .output(self.outputs.iter())
            .action()
    }
}

#[derive(Default)]
pub(super) struct RuleProcess {
    pub(super) input_vertices: HashMap<String, VertexId>,
    pub(super) new_vertices: HashSet<String>,
    pub(super) new_edges: Vec<ArrowConstraint<String>>,
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
                self.input_vertices
                    .entry(label.to_owned())
                    .or_insert(candidates[0]);
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

    pub fn output<'a>(
        mut self,
        outputs: impl Iterator<Item = &'a RuleObject<String>> + 'a,
    ) -> Self {
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

    pub fn action(self) -> GraphAction {
        let input_vertices: Vec<_> = self.input_vertices.values().copied().collect();
        let new_vertices = self.new_vertices.len();
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

        GraphAction::ApplyRule {
            input_vertices,
            new_vertices,
            new_edges,
        }
    }
}

// pub(super) fn rule_input<'a>(
//     graph: &Graph,
//     inputs: impl Iterator<Item = &'a RuleObject<String>>,
//     selection: impl Iterator<Item = &'a GraphObject>,
// ) -> RuleProcess {
//     let mut input_vertices = Vec::new();
//     let mut input_map = HashMap::new();

//     for input in inputs.zip(selection) {
//         let mut insert_vertex = |label: &str, id: VertexId| {
//             input_map.entry(label.to_owned()).or_insert_with(|| {
//                 input_vertices.push(id);
//                 input_vertices.len() - 1
//             });
//         };

//         match input {
//             (RuleObject::Vertex { label }, &GraphObject::Vertex { id }) => {
//                 insert_vertex(label, id);
//             }
//             (RuleObject::Edge { constraint, .. }, &GraphObject::Edge { id }) => {
//                 let edge = &graph.graph.edges.get(&id).unwrap().edge;
//                 insert_vertex(&constraint.from, edge.from);
//                 insert_vertex(&constraint.to, edge.to);
//             }
//             _ => unreachable!("Must be an error in Rule::check_input"),
//         };
//     }

//     (input_vertices, input_map)
// }

// pub(super) fn rule_infer<'a>(
//     graph: &Graph,
//     infers: impl Iterator<Item = &'a RuleObject<String>> + 'a,
//     input_vertices: &mut Vec<VertexId>,
//     input_map: &mut HashMap<String, usize>,
//     input_edges: &mut Vec<EdgeId>,
// ) -> (HashMap<String, usize>, Vec<ArrowConstraint<usize>>) {
//     // A collection of all vertices from the graph that satisfy the inferring constraints.
//     let mut inferred_vertices = HashMap::new();

//     /// Get information about the vertex
//     fn infer_vertex<'a>(
//         graph: &'a Graph,
//         label: &str,
//         inferred_vertices: &'a mut HashMap<String, (Vec<VertexId>, Vec<ArrowConstraint<String>>)>,
//         rule_vertices: &Vec<VertexId>,
//         rule_map: &HashMap<String, usize>,
//         connection: Option<ArrowConstraint<String>>,
//     ) -> &'a mut Vec<VertexId> {
//         let (vertices, edges) = inferred_vertices
//             .entry(label.to_owned())
//             .or_insert_with(|| {
//                 (
//                     rule_map
//                         .get(label)
//                         .map(|&index| vec![rule_vertices[index]])
//                         .unwrap_or_else(|| {
//                             graph.graph.vertices.iter().map(|(&id, _)| id).collect()
//                         }),
//                     vec![],
//                 )
//             });
//         if let Some(connection) = connection {
//             edges.push(connection);
//         }
//         vertices
//     }

//     // Filter vertices
//     for infer in infers {
//         match infer {
//             RuleObject::Vertex { label } => {
//                 infer_vertex(
//                     graph,
//                     label,
//                     &mut inferred_vertices,
//                     &input_vertices,
//                     &input_map,
//                     None,
//                 );
//             }
//             RuleObject::Edge { constraint, .. } => {
//                 // Check from
//                 let infer_to: Vec<_> = infer_vertex(
//                     graph,
//                     &constraint.to,
//                     &mut inferred_vertices,
//                     &input_vertices,
//                     &input_map,
//                     Some(constraint.clone()),
//                 )
//                 .iter()
//                 .copied()
//                 .collect();

//                 let infer_from = infer_vertex(
//                     graph,
//                     &constraint.from,
//                     &mut inferred_vertices,
//                     &input_vertices,
//                     &input_map,
//                     Some(constraint.clone()),
//                 );

//                 infer_from.retain(|from| {
//                     graph.graph.edges.iter().any(|(_, edge)| {
//                         let end_points = edge.end_points();
//                         edge.edge
//                             .connection
//                             .check_constraint(&constraint.connection)
//                             && (end_points[0].eq(from) && infer_to.contains(end_points[1]))
//                     })
//                 });

//                 // Check to
//                 let infer_from: Vec<_> = infer_from.iter().copied().collect();

//                 let infer_to = infer_vertex(
//                     graph,
//                     &constraint.to,
//                     &mut inferred_vertices,
//                     &input_vertices,
//                     &input_map,
//                     None,
//                 );

//                 infer_to.retain(|to| {
//                     graph.graph.edges.iter().any(|(_, edge)| {
//                         let end_points = edge.end_points();
//                         edge.edge
//                             .connection
//                             .check_constraint(&constraint.connection)
//                             && (end_points[1].eq(to) && infer_from.contains(end_points[0]))
//                     })
//                 });
//             }
//         }
//     }

//     let mut new_connections = Vec::new();

//     for (label, (candidates, edges)) in inferred_vertices {
//         if !candidates.is_empty() {
//             // Inferred a vertex -> add it to the list
//             input_map.entry(label.to_owned()).or_insert_with(|| {
//                 input_vertices.push(candidates[0]);
//                 input_vertices.len() - 1
//             });
//         } else {
//             // New vertex, new edges
//             new_connections.extend(edges);
//         }
//     }

//     let mut new_vertices_map = HashMap::new();
//     let mut get_vertex = |label: &str| {
//         input_map.get(label).map(|&index| index).unwrap_or_else(|| {
//             let len = new_vertices_map.len();
//             *new_vertices_map.entry(label.to_owned()).or_insert(len) + input_map.len()
//         })
//     };

//     let mut new_edges: Vec<_> = new_connections
//         .into_iter()
//         .map(|edge| {
//             let from = get_vertex(&edge.from);
//             let to = get_vertex(&edge.to);
//             ArrowConstraint::new(from, to, edge.connection)
//         })
//         .collect();

//     new_edges.sort();
//     new_edges.dedup();

//     (new_vertices_map, new_edges)
// }

// fn rule_output<'a>(
//     outputs: impl Iterator<Item = &'a RuleObject<String>> + 'a,
//     input_map: &HashMap<String, usize>,
//     new_vertices_map: &mut HashMap<String, usize>,
//     new_edges: &mut Vec<ArrowConstraint<usize>>,
// ) {
//     for output in outputs {
//         match output {
//             RuleObject::Vertex { label } => {
//                 new_vertices_map.insert(label.to_owned(), new_vertices_map.len());
//             }
//             RuleObject::Edge { constraint, .. } => {
//                 let mut get_vertex = |label: &str| {
//                     input_map
//                         .get(label)
//                         .map(|&index| index)
//                         .or_else(|| {
//                             new_vertices_map
//                                 .get(label)
//                                 .map(|&index| index + input_map.len())
//                         })
//                         .unwrap_or_else(|| {
//                             new_vertices_map.insert(label.to_owned(), new_vertices_map.len());
//                             new_vertices_map.len() + input_map.len() - 1
//                         })
//                 };

//                 let from = get_vertex(&constraint.from);
//                 let to = get_vertex(&constraint.to);
//                 new_edges.push(ArrowConstraint::new(from, to, constraint.connection));
//             }
//         }
//     }
// }

/// Color:
///  - Ok(None)        -> Do nothing or use default color (inferred from context)
///  - Ok(Some(color)) -> Override existing color
///  - Err(color)      -> Create new with the given color
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
            body: ForceBody::new(random_shift(), POINT_MASS),
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
