use super::*;

pub struct RuleSelection {
    rule_index: usize,
    rule_input: Vec<GraphObject>,
    current_selection: usize,
    selection: Vec<GraphObject>,
    inferred_options: Option<Vec<GraphObject>>,
}

impl RuleSelection {
    pub fn new(graph: &Graph, rule_index: usize, rules: &Rules) -> Self {
        let mut selection = RuleSelection {
            rule_input: rules
                .get_rule(rule_index)
                .unwrap()
                .get_input()
                .iter()
                .copied()
                .collect(),
            selection: Vec::new(),
            inferred_options: None,
            current_selection: 0,
            rule_index,
        };
        selection.infer_current(graph, rules);
        selection
    }

    pub fn rule(&self) -> usize {
        self.rule_index
    }

    pub fn current(&self) -> Option<&GraphObject> {
        self.rule_input.get(self.current_selection)
    }

    /// Select a vertex. Returns either the next vertex
    /// from the rule graph to select or None.
    pub fn select(
        &mut self,
        graph: &Graph,
        selection: GraphObject,
        rules: &Rules,
    ) -> Option<&GraphObject> {
        if self.current_selection >= self.rule_input.len() {
            return None;
        }

        self.selection.push(selection);
        self.current_selection += 1;
        self.infer_current(graph, rules);
        self.current()
    }

    pub fn selection(&self) -> &Vec<GraphObject> {
        &self.selection
    }

    pub fn inferred_options(&self) -> &Option<Vec<GraphObject>> {
        &self.inferred_options
    }

    /// Infer possible selections for the current rule selection
    pub fn infer_current(&mut self, graph: &Graph, rules: &Rules) {
        let rule = rules.get_rule(self.rule()).unwrap();
        let selected = self.selection.len();

        let process = RuleProcess::input(
            graph,
            rule.inputs().iter().take(selected),
            self.selection.iter(),
        )
        .constraint(graph, rule.constraints());

        let process = match process {
            Ok(process) => process,
            Err(_) => {
                self.inferred_options = None;
                return;
            }
        };

        let candidates = process.infer_candidates(graph, &rule.inputs()[selected..], 1);

        let next = match rule.inputs().get(selected) {
            Some(next) => next,
            None => return,
        };
        match next {
            RuleObject::Vertex { label } => {
                self.inferred_options = candidates.get(label).map(|(vertices, _)| {
                    vertices
                        .iter()
                        .map(|&id| GraphObject::Vertex { id })
                        .collect()
                });
            }
            RuleObject::Edge { constraint, .. } => {
                self.inferred_options = candidates.get(&constraint.from).and_then(|(from, _)| {
                    candidates.get(&constraint.to).map(|(to, _)| {
                        graph
                            .graph
                            .edges
                            .iter()
                            .filter(|(_, edge)| {
                                from.contains(&edge.edge.from) && to.contains(&edge.edge.to)
                            })
                            .map(|(&id, _)| GraphObject::Edge { id })
                            .collect()
                    })
                });
            }
        }
    }
}
