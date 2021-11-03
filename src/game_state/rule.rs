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
                self.force_graph
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
            .filter_map(|id| self.force_graph.graph.edges.get(id).map(|edge| &edge.edge))
            .collect();

        // Check & apply the rule
        let rule = &self.rules[rule_index];
        rule.check_constraints(&input_vertices, &input_edges)
            .map(|vertices| rule.apply(&mut self.force_graph, vertices))
            .is_some()
    }
}

pub struct Rule {
    input_vertices: usize,
    input_edges: Vec<ArrowConstraint<usize>>,
    new_vertices: usize,
    new_edges: Vec<Arrow<usize>>,
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

        Ok(Self {
            input_vertices,
            input_edges,
            new_vertices,
            new_edges,
        })
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
        for _ in 0..self.new_vertices {
            vertices.push(graph.graph.new_vertex(ForceVertex {
                is_anchor: false,
                body: ForceBody {
                    position: Vec2::ZERO,
                    mass: POINT_MASS,
                    velocity: Vec2::ZERO,
                },
                vertex: Point {
                    radius: POINT_RADIUS,
                    color: Color::WHITE,
                },
            }))
        }

        // Add connections
        for new_edge in &self.new_edges {
            let new_edge = Arrow {
                from: vertices[new_edge.from],
                to: vertices[new_edge.to],
                connection: new_edge.connection,
            };
            graph
                .graph
                .new_edge(ForceEdge::new(
                    Vec2::ZERO,
                    Vec2::ZERO,
                    ARROW_BODIES,
                    ARROW_MASS,
                    new_edge,
                ))
                .expect("Attempted to connect a non-existent vertex when applying a rule");
        }
    }
}
