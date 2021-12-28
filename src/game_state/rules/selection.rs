use super::*;

#[derive(Debug)]
pub struct RuleSelection {
    rule_index: usize,
    rule_input: Vec<GraphObject>,
    current_selection: usize,
    selection: Vec<GraphObject>,
    inferred_options: Option<Vec<GraphObject>>,
    inverse: bool,
}

impl RuleSelection {
    pub fn new(graph: &Graph, rule_index: usize, rules: &Rules, inverse: bool) -> Self {
        let mut selection = RuleSelection {
            rule_input: match inverse {
                false => rules[rule_index].graph_input(),
                true => rules[rule_index].inverse_graph_input(),
            }
            .iter()
            .copied()
            .collect(),
            selection: Vec::new(),
            inferred_options: None,
            current_selection: 0,
            rule_index,
            inverse,
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

    /// Select a vertex. Returns the next vertex
    /// from the rule graph to select.
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
    fn infer_current(&mut self, graph: &Graph, rules: &Rules) {
        let construction = rules.get(self.rule()).and_then(|rule| match self.inverse {
            false => rule.statement().first(),
            true => rule.inverse_statement().first(),
        });
        self.inferred_options = construction
            .and_then(|construction| infer_construction(construction, graph, &self.selection));
    }
}

fn infer_construction(
    construction: &RuleConstruction,
    graph: &Graph,
    selection: &Vec<GraphObject>,
) -> Option<Vec<GraphObject>> {
    let input_constraints = match construction {
        RuleConstruction::Forall(constraints) | RuleConstruction::Exists(constraints) => {
            constraints
        }
    };
    let mut constraints = input_constraints.iter();

    let mut bindings = Bindings::new();
    for selected in selection {
        let constraint = constraints.next().unwrap();
        match constraint {
            Constraint::RuleObject(label, object) => match (selected, object) {
                (GraphObject::Vertex { id }, RuleObject::Vertex { .. }) => {
                    bindings.bind_object(label.to_owned(), *id);
                }
                (GraphObject::Edge { id }, RuleObject::Edge { constraint }) => {
                    bindings.bind_morphism(label.to_owned(), *id);
                    let edge = graph.graph.edges.get(id).unwrap();
                    bindings.bind_object(constraint.from.to_owned(), edge.edge.from);
                    bindings.bind_object(constraint.to.to_owned(), edge.edge.to);
                }
                _ => {
                    return None;
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
            return None;
        }
    };

    find_candidates(input_constraints, &bindings, graph).map(|options| {
        options
            .map(|binds| match next_object {
                RuleObject::Vertex { .. } => GraphObject::Vertex {
                    id: binds.get_object(next_label).unwrap(),
                },
                RuleObject::Edge { .. } => GraphObject::Edge {
                    id: binds.get_morphism(next_label).unwrap(),
                },
            })
            .collect()
    })
}
