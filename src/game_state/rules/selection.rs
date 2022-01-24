use super::*;

#[derive(Debug)]
pub struct RuleSelection {
    rule_index: usize,
    rule_input: Vec<CategoryThing>,
    current_selection: usize,
    selection: Vec<CategoryThing>,
    inferred_options: Option<Vec<CategoryThing>>,
    inverse: bool,
}

impl RuleSelection {
    pub fn new(
        category: &Category,
        equalities: &Equalities,
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
        selection.infer_current(category, equalities, rules);
        selection
    }

    pub fn inverse(&self) -> bool {
        self.inverse
    }

    pub fn rule(&self) -> usize {
        self.rule_index
    }

    pub fn current(&self) -> Option<&CategoryThing> {
        self.rule_input.get(self.current_selection)
    }

    /// Select a vertex. Returns the next vertex
    /// from the rule graph to select.
    pub fn select(
        &mut self,
        category: &Category,
        equalities: &Equalities,
        selection: CategoryThing,
        rules: &Rules,
    ) -> Option<&CategoryThing> {
        if self.current_selection >= self.rule_input.len() {
            return None;
        }

        self.selection.push(selection);
        self.current_selection += 1;
        self.infer_current(category, equalities, rules);

        self.current()
    }

    pub fn selection(&self) -> &Vec<CategoryThing> {
        &self.selection
    }

    pub fn inferred_options(&self) -> &Option<Vec<CategoryThing>> {
        &self.inferred_options
    }

    /// Infer possible selections for the current rule selection
    fn infer_current(&mut self, category: &Category, equalities: &Equalities, rules: &Rules) {
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
                    category,
                    equalities,
                    &self.selection,
                )
            })
        });
    }
}

fn infer_construction(
    input_constraints: &Constraints,
    all_constraints: &Constraints,
    category: &Category,
    equalities: &Equalities,
    selection: &Vec<CategoryThing>,
) -> Vec<CategoryThing> {
    let mut input_constraints = input_constraints.iter();

    let mut bindings = Bindings::new();
    for selected in selection {
        let constraint = input_constraints.next().unwrap();
        match constraint {
            Constraint::RuleObject(label, object) => match (selected, object) {
                (CategoryThing::Object { id }, RuleObject::Object { .. }) => {
                    bindings.bind_object(label.clone(), *id);
                }
                (CategoryThing::Morphism { id }, RuleObject::Morphism { constraint }) => {
                    bindings.bind_morphism(label.clone(), *id);
                    let morphism = category.morphisms.get(id).unwrap();

                    match (morphism.connection, &constraint.connection) {
                        (
                            MorphismConnection::Regular { from, to },
                            MorphismConnection::Regular {
                                from: constraint_from,
                                to: constraint_to,
                            },
                        ) => {
                            bindings.bind_object(constraint_from.clone(), from);
                            bindings.bind_object(constraint_to.clone(), to);
                        }
                        (
                            MorphismConnection::Isomorphism(a, b),
                            MorphismConnection::Isomorphism(constraint_a, constraint_b),
                        ) => {
                            bindings.bind_object(constraint_a.clone(), a);
                            bindings.bind_object(constraint_b.clone(), b);
                        }
                        _ => return vec![],
                    }
                }
                _ => return vec![],
            },
            Constraint::MorphismEq(_, _) => {
                // TODO: select equalities either from the list, or by clicking on morphisms
            }
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

    find_candidates(all_constraints, &bindings, category, equalities)
        .map(|candidates| {
            candidates
                .into_iter()
                .map(|binds| match next_object {
                    RuleObject::Object { .. } => CategoryThing::Object {
                        id: binds.get_object(next_label).expect(
                            "An object was expected to be inferred, does it not have a name?",
                        ),
                    },
                    RuleObject::Morphism { .. } => CategoryThing::Morphism {
                        id: binds.get_morphism(next_label).expect(
                            "A morphism was expected to be inferred, does it not have a name?",
                        ),
                    },
                })
                .collect()
        })
        .unwrap_or_default()
}
