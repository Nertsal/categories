use super::*;

pub struct RuleBuilder {
    input_vertices: usize,
    input_edges: Vec<ArrowConstraint<usize>>,
    new_vertices: usize,
    new_edges: Vec<Arrow<usize>>,
}

impl RuleBuilder {
    pub fn new(input_vertices: usize) -> Self {
        Self {
            input_vertices,
            new_vertices: 0,
            input_edges: Vec::new(),
            new_edges: Vec::new(),
        }
    }

    pub fn with_edge_constraints(
        mut self,
        input_edges: impl Iterator<Item = ArrowConstraint<usize>>,
    ) -> Self {
        self.input_edges.extend(input_edges);
        self
    }

    pub fn with_new_vertices(mut self, new_vertices: usize) -> Self {
        self.new_vertices += new_vertices;
        self
    }

    pub fn with_new_edges(mut self, new_edges: impl Iterator<Item = Arrow<usize>>) -> Self {
        self.new_edges.extend(new_edges);
        self
    }

    pub fn build(self) -> Rule {
        Rule::new(
            self.input_vertices,
            self.input_edges,
            self.new_vertices,
            self.new_edges,
        )
        .unwrap()
    }
}
