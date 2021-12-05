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
                .graph_input()
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

        let input_constraints =
            match rule
                .statement()
                .first()
                .and_then(|construction| match construction {
                    RuleConstruction::Forall(constraints)
                    | RuleConstruction::Exists(constraints) => Some(constraints),
                    RuleConstruction::SuchThat => None,
                }) {
                Some(constraints) => constraints,
                None => {
                    self.inferred_options = None;
                    return;
                }
            };
        let mut constraints = input_constraints.iter();

        let mut bindings = Bindings::new();
        for selected in &self.selection {
            let constraint = constraints.next().unwrap();
            match constraint {
                Constraint::RuleObject(label, object) => match (selected, object) {
                    (GraphObject::Vertex { id }, RuleObject::Vertex) => {
                        bindings.bind_object(label.to_owned(), *id);
                    }
                    (GraphObject::Edge { id }, RuleObject::Edge { constraint }) => {
                        bindings.bind_morphism(label.to_owned(), *id);
                        let edge = graph.graph.edges.get(id).unwrap();
                        bindings.bind_object(constraint.from.to_owned(), edge.edge.from);
                        bindings.bind_object(constraint.to.to_owned(), edge.edge.to);
                    }
                    _ => {
                        self.inferred_options = None;
                        return;
                    }
                },
                Constraint::MorphismEq(_, _) => (),
            }
        }

        let next = match constraints.next() {
            Some(Constraint::RuleObject(label, object)) => Some((label, object)),
            _ => None,
        };
        let (next_label, next_object) = match next {
            Some(next) => next,
            None => {
                self.inferred_options = None;
                return;
            }
        };

        let options = find_candidates(input_constraints, &bindings, graph).map(|options| {
            options
                .map(|binds| match next_object {
                    RuleObject::Vertex => GraphObject::Vertex {
                        id: binds.get_object(next_label).unwrap(),
                    },
                    RuleObject::Edge { .. } => GraphObject::Edge {
                        id: binds.get_morphism(next_label).unwrap(),
                    },
                })
                .collect()
        });

        self.inferred_options = options;

        // let process = RuleProcess::input(
        //     graph,
        //     rule.inputs().iter().take(selected),
        //     self.selection.iter(),
        // );

        // let constraints = match process.constraint(graph, rule.constraints()) {
        //     Ok(constraints) => constraints,
        //     Err(_) => {
        //         self.inferred_options = None;
        //         return;
        //     }
        // };

        // let candidates =
        //     process.infer_candidates(graph, constraints, &rule.inputs()[selected..], 2);

        // let next = match rule.inputs().get(selected) {
        //     Some(next) => next,
        //     None => {
        //         self.inferred_options = None;
        //         return;
        //     }
        // };

        // match next {
        //     RuleObject::Vertex { label } => {
        //         self.inferred_options = candidates.get(label).map(|(vertices, _)| {
        //             vertices
        //                 .iter()
        //                 .map(|&id| GraphObject::Vertex { id })
        //                 .collect()
        //         });
        //     }
        //     RuleObject::Edge { constraint, .. } => {
        //         self.inferred_options = candidates.get(&constraint.from).and_then(|(from, _)| {
        //             candidates.get(&constraint.to).map(|(to, _)| {
        //                 graph
        //                     .graph
        //                     .edges
        //                     .iter()
        //                     .filter(|(_, edge)| {
        //                         from.contains(&edge.edge.from) && to.contains(&edge.edge.to)
        //                     })
        //                     .map(|(&id, _)| GraphObject::Edge { id })
        //                     .collect()
        //             })
        //         });
        //     }
        // }
    }
}
