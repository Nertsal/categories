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
    pub fn new(
        graph: &Graph,
        graph_equalities: &GraphEqualities,
        rule_index: usize,
        rules: &Rules,
        inverse: bool,
    ) -> Self {
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
        selection.infer_current(graph, graph_equalities, rules);
        selection
    }

    pub fn inverse(&self) -> bool {
        self.inverse
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
        graph_equalities: &GraphEqualities,
        selection: GraphObject,
        rules: &Rules,
    ) -> Option<&GraphObject> {
        if self.current_selection >= self.rule_input.len() {
            return None;
        }

        self.selection.push(selection);
        self.current_selection += 1;
        self.infer_current(graph, graph_equalities, rules);

        self.current()
    }

    pub fn selection(&self) -> &Vec<GraphObject> {
        &self.selection
    }

    pub fn inferred_options(&self) -> &Option<Vec<GraphObject>> {
        &self.inferred_options
    }

    /// Infer possible selections for the current rule selection
    fn infer_current(&mut self, graph: &Graph, graph_equalities: &GraphEqualities, rules: &Rules) {
        let constraints = rules.get(self.rule()).map(|rule| {
            let statement = match self.inverse {
                false => rule.statement().iter(),
                true => rule.inverse_statement().iter(),
            };
            statement.map_while(|construction| match construction {
                RuleConstruction::Forall(constraints) => Some(constraints),
                RuleConstruction::Exists(_) => None,
            })
        });

        self.inferred_options = constraints.and_then(|mut constraints| {
            constraints.next().map(|construction| {
                let constraints = construction
                    .iter()
                    .chain(constraints.flat_map(|x| x.iter()))
                    .cloned()
                    .collect();
                infer_construction(
                    construction,
                    &constraints,
                    graph,
                    graph_equalities,
                    &self.selection,
                )
            })
        });
    }
}

fn infer_construction(
    input_constraints: &Constraints,
    all_constraints: &Constraints,
    graph: &Graph,
    graph_equalities: &GraphEqualities,
    selection: &Vec<GraphObject>,
) -> Vec<GraphObject> {
    let mut input_constraints = input_constraints.iter();

    let mut bindings = Bindings::new();
    for selected in selection {
        let constraint = input_constraints.next().unwrap();
        match constraint {
            Constraint::RuleObject(label, object) => match (selected, object) {
                (GraphObject::Vertex { id }, RuleObject::Vertex { .. }) => {
                    bindings.bind_object(label.clone(), *id);
                }
                (GraphObject::Edge { id }, RuleObject::Edge { constraint }) => {
                    bindings.bind_morphism(label.clone(), *id);
                    let edge = graph.graph.edges.get(id).unwrap();
                    bindings.bind_object(constraint.from.clone(), edge.edge.from);
                    bindings.bind_object(constraint.to.clone(), edge.edge.to);
                }
                _ => {
                    return vec![];
                }
            },
            Constraint::MorphismEq(_, _) => (),
        }
    }

    let next = match input_constraints.next() {
        Some(Constraint::RuleObject(label, object)) => Some((label, object)),
        _ => None,
    };
    let (next_label, next_object) = match next {
        Some(next) => next,
        None => {
            return vec![];
        }
    };

    find_candidates(all_constraints, &bindings, graph, graph_equalities)
        .map(|candidates| {
            candidates
                .into_iter()
                .map(|binds| match next_object {
                    RuleObject::Vertex { .. } => GraphObject::Vertex {
                        id: binds.get_object(next_label).expect(
                            "An object was expected to be inferred, does it not have a name?",
                        ),
                    },
                    RuleObject::Edge { .. } => GraphObject::Edge {
                        id: binds.get_morphism(next_label).expect(
                            "A morphism was expected to be inferred, does it not have a name?",
                        ),
                    },
                })
                .collect()
        })
        .unwrap_or_default()
}
