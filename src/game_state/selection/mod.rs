use super::*;

mod geom;
mod select;

use geom::*;
pub use select::*;

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
        rule_index: usize,
        rules: &Vec<RenderableRule>,
        inverse: bool,
    ) -> Self {
        let mut selection = RuleSelection {
            rule_input: match inverse {
                false => rules[rule_index].inner.get_input(),
                true => todo!(), //rules[rule_index].inverse_graph_input(),
            }
            .iter()
            .filter_map(|constraint| match constraint {
                category::Constraint::Object { label, tags } => {
                    Some(CategoryThing::Object { id: todo!() })
                }
                category::Constraint::Morphism {
                    label,
                    connection,
                    tags,
                } => Some(CategoryThing::Morphism { id: todo!() }),
                category::Constraint::Equality(_, _) | category::Constraint::Commute { .. } => None,
            })
            .collect(),
            selection: Vec::new(),
            inferred_options: None,
            current_selection: 0,
            rule_index,
            inverse,
        };
        selection.infer_current(category, rules);
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
        selection: CategoryThing,
        rules: &Vec<RenderableRule>,
    ) -> Option<&CategoryThing> {
        if self.current_selection >= self.rule_input.len() {
            return None;
        }

        self.selection.push(selection);
        self.current_selection += 1;
        self.infer_current(category, rules);

        self.current()
    }

    pub fn selection(&self) -> &Vec<CategoryThing> {
        &self.selection
    }

    pub fn inferred_options(&self) -> &Option<Vec<CategoryThing>> {
        &self.inferred_options
    }

    /// Infer possible selections for the current rule selection
    fn infer_current(&mut self, category: &Category, rules: &Vec<RenderableRule>) {
        let constraints = rules.get(self.rule()).map(|rule| {
            let statement = match self.inverse {
                false => rule.inner.get_statement().iter(),
                true => todo!(), //rule.inverse_statement().iter(),
            };
            statement.map_while(|construction| match construction {
                category::RuleConstruction::Forall(constraints) => Some(constraints),
                category::RuleConstruction::Exists(_) => None,
            })
        });

        self.inferred_options = constraints.and_then(|mut constraints| {
            constraints.next().map(|construction| {
                let constraints = construction
                    .iter()
                    .chain(constraints.flat_map(|x| x.iter()))
                    .cloned()
                    .collect();
                infer_construction(construction, &constraints, category, &self.selection)
            })
        });
    }
}

fn infer_construction(
    input_constraints: &Constraints,
    all_constraints: &Constraints,
    category: &Category,
    selection: &Vec<CategoryThing>,
) -> Vec<CategoryThing> {
    use category::Constraint;

    let mut input_constraints = input_constraints.iter();

    let mut bindings = Bindings::new();
    for selected in selection {
        let constraint = input_constraints.next().unwrap();
        match (constraint, selected) {
            (Constraint::Object { label, .. }, CategoryThing::Object { id }) => {
                bindings.bind_object(label.clone(), *id);
            }
            (Constraint::Object { .. }, _) => (),
            (
                Constraint::Morphism {
                    label,
                    connection,
                    tags,
                },
                CategoryThing::Morphism { id },
            ) => {
                bindings.bind_morphism(label.clone(), *id);
                let morphism = category.morphisms.get(id).unwrap();

                match (morphism.connection, connection) {
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
            (Constraint::Morphism { .. }, _) => (),
            (Constraint::Equality(_, _), _) => {
                // TODO: select equalities either from the list, or by clicking on morphisms
            }
            (Constraint::Commute { f, g, h }, _) => {
                // TODO: similar to equalities
            }
        }
    }

    let next = match input_constraints.next() {
        Some(x) => x,
        None => return vec![],
    };

    category
        .find_candidates(all_constraints, &bindings)
        .map(|candidates| {
            candidates
                .into_iter()
                .filter_map(|binds| match next {
                    Constraint::Object { label, .. } => Some(CategoryThing::Object {
                        id: binds.get_object(label).expect(
                            "An object was expected to be inferred, does it not have a name?",
                        ),
                    }),
                    Constraint::Morphism { label, .. } => Some(CategoryThing::Morphism {
                        id: binds.get_morphism(label).expect(
                            "A morphism was expected to be inferred, does it not have a name?",
                        ),
                    }),
                    _ => None,
                })
                .collect()
        })
        .unwrap_or_default()
}
