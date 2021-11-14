use super::*;

pub struct RuleSelection {
    rule_index: usize,
    rule_vertices: Vec<VertexId>,
    current_selection: usize,
    selected_vertices: Vec<VertexId>,
}

impl RuleSelection {
    pub fn new(rule_index: usize, rule: &Rule) -> Self {
        let rule_vertices = rule
            .graph()
            .graph
            .vertices
            .iter()
            .map(|(&id, _)| id)
            .collect::<Vec<_>>();
        RuleSelection {
            selected_vertices: Vec::with_capacity(rule_vertices.len()),
            current_selection: 0,
            rule_vertices,
            rule_index,
        }
    }

    pub fn rule(&self) -> usize {
        self.rule_index
    }

    pub fn current(&self) -> usize {
        self.current_selection
    }

    /// Select a vertex. Returns either the next vertex
    /// from the rule graph to select or None.
    pub fn select(&mut self, vertex: VertexId) -> Option<&VertexId> {
        assert!(
            self.current_selection < self.rule_vertices.len(),
            "Tried to select more vertices than needed"
        );

        self.selected_vertices.push(vertex);
        self.current_selection += 1;
        self.rule_vertices.get(self.current_selection)
    }

    pub fn rule_vertices(&self) -> &Vec<VertexId> {
        &self.rule_vertices
    }

    pub fn selection(&self) -> &Vec<VertexId> {
        &self.selected_vertices
    }

    pub fn to_selection(self) -> Vec<VertexId> {
        self.selected_vertices
    }
}
