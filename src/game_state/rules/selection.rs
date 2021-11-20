use super::*;

pub struct RuleSelection {
    rule_index: usize,
    rule_input: Vec<GraphObject>,
    current_selection: usize,
    selection: Vec<GraphObject>,
}

impl RuleSelection {
    pub fn new(rule_index: usize, rule: &Rule) -> Self {
        RuleSelection {
            rule_input: rule.get_input().iter().copied().collect(),
            selection: Vec::new(),
            current_selection: 0,
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
    pub fn select(&mut self, selection: GraphObject) -> Option<&GraphObject> {
        if self.current_selection >= self.rule_input.len() {
            return None;
        }

        self.selection.push(selection);
        self.current_selection += 1;
        self.rule_input.get(self.current_selection)
    }

    pub fn rule_input(&self) -> &Vec<GraphObject> {
        &self.rule_input
    }

    pub fn selection(&self) -> &Vec<GraphObject> {
        &self.selection
    }

    pub fn to_selection(self) -> Vec<GraphObject> {
        self.selection
    }
}