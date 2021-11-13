use super::*;

impl GameState {
    /// Attempts to apply a rule.
    /// Returns whether the rule was applied successfully.
    pub fn apply_rule(&mut self, rule_index: usize) -> bool {
        // Collect input
        let input_vertices = self
            .selection
            .vertices
            .iter()
            .filter_map(|id| {
                self.main_graph
                    .graph
                    .vertices
                    .get(id)
                    .map(|vertex| (id, &vertex.vertex))
            })
            .collect();
        let input_edges = self
            .selection
            .edges
            .iter()
            .filter_map(|id| self.main_graph.graph.edges.get(id).map(|edge| &edge.edge))
            .collect();

        // Check & apply the rule
        let rule = self.rules.get_rule(rule_index).unwrap();
        rule.check_constraints(&input_vertices, &input_edges)
            .map(|vertices| rule.apply(&mut self.main_graph, vertices))
            .is_some()
    }
}

pub struct Rule {
    input_vertices: usize,
    input_edges: Vec<ArrowConstraint<usize>>,
    new_vertices: Vec<VertexId>,
    new_edges: Vec<Arrow<usize>>,
    graph: Graph,
}

impl Rule {
    pub fn new(
        input_vertices: usize,
        input_edges: Vec<ArrowConstraint<usize>>,
        new_vertices: usize,
        new_edges: Vec<Arrow<usize>>,
    ) -> Result<Self, String> {
        // Check input_edges
        for input_edge in &input_edges {
            if input_edge.from >= input_vertices || input_edge.to >= input_vertices {
                return Err(format!(
                    "Invalid input constraint: required edge ({}, {}), however, only {} vertices are required.",
                    input_edge.from, input_edge.to, input_vertices
                ));
            }
        }

        // Check new edges
        for new_edge in &new_edges {
            let available = input_vertices + new_vertices;
            if new_edge.from >= available || new_edge.to >= available {
                return Err(format!("Invalid output result: attempted to connect ({}, {}), however, only {} vertices are available", new_edge.from, new_edge.to, available));
            }
        }

        let mut rng = thread_rng();
        let mut random_pos = || vec2(rng.gen(), rng.gen());
        let mut graph = Graph::new(ForceParameters::default());
        // Vertices
        let mut i = 0;
        let mut new_vertex = || {
            let id = graph.graph.new_vertex(ForceVertex {
                is_anchor: false,
                body: ForceBody::new(random_pos(), POINT_MASS),
                vertex: Point {
                    label: i.to_string(),
                    radius: POINT_RADIUS,
                    color: Color::WHITE,
                },
            });
            i += 1;
            id
        };
        let input_vertex_ids = (0..input_vertices)
            .map(|_| new_vertex())
            .collect::<Vec<_>>();
        let new_vertex_ids = (0..new_vertices).map(|_| new_vertex()).collect::<Vec<_>>();
        // Edges
        for edge in &input_edges {
            graph.graph.new_edge(ForceEdge::new(
                random_pos(),
                random_pos(),
                ARROW_BODIES,
                ARROW_MASS,
                Arrow {
                    label: "".to_owned(),
                    from: input_vertex_ids[edge.from],
                    to: input_vertex_ids[edge.to],
                    connection: edge.connection,
                },
            ));
        }
        for edge in &new_edges {
            graph.graph.new_edge(ForceEdge::new(
                random_pos(),
                random_pos(),
                ARROW_BODIES,
                ARROW_MASS,
                Arrow {
                    label: "".to_owned(),
                    from: *input_vertex_ids
                        .get(edge.from)
                        .unwrap_or_else(|| &new_vertex_ids[edge.from - input_vertices]),
                    to: *input_vertex_ids
                        .get(edge.to)
                        .unwrap_or_else(|| &new_vertex_ids[edge.to - input_vertices]),
                    connection: edge.connection,
                },
            ));
        }

        Ok(Self {
            new_vertices: new_vertex_ids,
            graph,
            input_vertices,
            input_edges,
            new_edges,
        })
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
    /// Returns Some(input_vertices) if the rule can be applied, otherwise returns None.
    fn check_constraints(
        &self,
        input_vertices: &Vec<(&VertexId, &Point)>,
        input_edges: &Vec<&Arrow<VertexId>>,
    ) -> Option<Vec<VertexId>> {
        // Check vertices
        if input_vertices.len() != self.input_vertices {
            return None;
        }

        let vertices: Vec<_> = input_vertices.iter().map(|(&id, _)| id).collect();

        // Check edges
        for edge in &self.input_edges {
            let constraint = ArrowConstraint {
                from: vertices[edge.from],
                to: vertices[edge.to],
                connection: edge.connection,
            };
            if !input_edges
                .iter()
                .any(|edge| edge.check_constraint(&constraint))
            {
                return None;
            }
        }

        Some(vertices)
    }

    /// Applies the rule
    fn apply(&self, graph: &mut Graph, mut vertices: Vec<VertexId>) {
        // Apply rule
        // Spawn new vertices
        for id in &self.new_vertices {
            vertices.push(graph.graph.new_vertex(ForceVertex {
                is_anchor: false,
                body: ForceBody {
                    position: graph.graph.vertices.get(id).unwrap().body.position,
                    mass: POINT_MASS,
                    velocity: Vec2::ZERO,
                },
                vertex: Point {
                    label: "".to_owned(),
                    radius: POINT_RADIUS,
                    color: Color::WHITE,
                },
            }))
        }

        // Add connections
        for new_edge in &self.new_edges {
            let new_edge = Arrow {
                label: "".to_owned(),
                from: vertices[new_edge.from],
                to: vertices[new_edge.to],
                connection: new_edge.connection,
            };
            graph
                .graph
                .new_edge(ForceEdge::new(
                    graph
                        .graph
                        .vertices
                        .get(&new_edge.from)
                        .unwrap()
                        .body
                        .position,
                    graph
                        .graph
                        .vertices
                        .get(&new_edge.to)
                        .unwrap()
                        .body
                        .position,
                    ARROW_BODIES,
                    ARROW_MASS,
                    new_edge,
                ))
                .expect("Attempted to connect a non-existent vertex when applying a rule");
        }
    }
}
