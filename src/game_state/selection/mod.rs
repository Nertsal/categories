use super::*;

mod geom;
mod select;

use geom::*;
pub use select::*;

#[derive(Debug)]
pub struct RuleSelection {
    rule_index: usize,
    rule_input: Vec<RuleInput<Label>>,
    current_selection: usize,
    selected: Bindings,
    inferred_options: Option<Vec<RuleInput<Label>>>,
    inverse: Option<usize>,
}

impl RuleSelection {
    pub fn new(
        category: &Category,
        rule_index: usize,
        rules: &Vec<RenderableRule>,
        inverse: Option<usize>,
    ) -> Self {
        let rule = &rules[rule_index];
        let rule_input = match inverse {
            None => rule.input.clone(),
            Some(_) => rule.inverse_input.clone(),
        };

        let mut selection = RuleSelection {
            selected: Bindings::new(),
            inferred_options: None,
            current_selection: 0,
            rule_input,
            rule_index,
            inverse,
        };
        selection.infer_current(category, rules);
        selection
    }

    pub fn inverse(&self) -> Option<usize> {
        self.inverse
    }

    pub fn rule(&self) -> usize {
        self.rule_index
    }

    pub fn current(&self) -> Option<&RuleInput<Label>> {
        self.rule_input.get(self.current_selection)
    }

    pub fn get_bindings(&self) -> &Bindings {
        &self.selected
    }

    /// Select an object/morphism. Returns the next object/morphism
    /// from the rule to select.
    pub fn select(
        &mut self,
        category: &Category,
        selection: RuleInput<Label>,
        rules: &Vec<RenderableRule>,
    ) -> Option<&RuleInput<Label>> {
        if self.current_selection >= self.rule_input.len() {
            return None;
        }

        match selection {
            RuleInput::Object { label, id } => {
                self.selected.bind_object(label, id);
            }
            RuleInput::Morphism { label, id } => {
                self.selected.bind_morphism(label, id);
            }
            RuleInput::Equality {
                label_f,
                id_f,
                label_g,
                id_g,
            } => {
                self.selected.bind_morphism(label_f, id_f);
                self.selected.bind_morphism(label_g, id_g);
            }
            RuleInput::Commute {
                label_f,
                id_f,
                label_g,
                id_g,
                label_h,
                id_h,
            } => {
                self.selected.bind_morphism(label_f, id_f);
                self.selected.bind_morphism(label_g, id_g);
                self.selected.bind_morphism(label_h, id_h);
            }
        }

        self.current_selection += 1;
        self.infer_current(category, rules);

        self.current()
    }

    pub fn inferred_options(&self) -> &Option<Vec<RuleInput<Label>>> {
        &self.inferred_options
    }

    /// Infer possible selections for the current rule selection
    fn infer_current(&mut self, category: &Category, rules: &Vec<RenderableRule>) {
        let constraints = rules.get(self.rule()).map(|rule| {
            let statement = match self.inverse {
                None => rule.inner.get_statement(),
                Some(inverse) => rule.inverse[inverse].get_statement(),
            };
            statement
                .iter()
                .map_while(|construction| match construction {
                    category::RuleConstruction::Forall(c) => Some(c),
                    category::RuleConstruction::Exists(_) => None,
                })
        });

        self.inferred_options = constraints.and_then(|mut constraints| {
            constraints.next().map(|construction| {
                let constraints = construction
                    .iter()
                    .chain(constraints.flat_map(|x| x))
                    .cloned()
                    .collect();
                construction
                    .get(self.current_selection)
                    .map(|constraint| {
                        infer_construction(constraint, &constraints, category, &self.selected)
                    })
                    .unwrap_or_default()
            })
        });
    }
}

fn infer_construction(
    input_constraint: &category::Constraint<Label>,
    all_constraints: &Constraints,
    category: &Category,
    bindings: &Bindings,
) -> Vec<RuleInput<Label>> {
    use category::Constraint;
    category
        .find_candidates(all_constraints, bindings)
        .map(|candidates| {
            candidates
                .into_iter()
                .map(|mut binds| {
                    binds.extend(bindings.clone());
                    match input_constraint {
                        Constraint::Object { label, .. } => RuleInput::Object {
                            label: label.clone(),
                            id: binds
                                .get_object(label)
                                .expect("An object could not be inferred"),
                        },
                        Constraint::Morphism { label, .. } => RuleInput::Morphism {
                            label: label.clone(),
                            id: binds
                                .get_morphism(label)
                                .expect("A morphism could not be inferred"),
                        },
                        Constraint::Equality(f, g) => RuleInput::Equality {
                            label_f: f.clone(),
                            label_g: g.clone(),
                            id_f: binds
                                .get_morphism(f)
                                .expect("A morphism could not be inferred"),
                            id_g: binds
                                .get_morphism(g)
                                .expect("A morphism could not be inferred"),
                        },
                        Constraint::Commute { f, g, h } => RuleInput::Commute {
                            label_f: f.clone(),
                            label_g: g.clone(),
                            label_h: h.clone(),
                            id_f: binds
                                .get_morphism(f)
                                .expect("A morphism could not be inferred"),
                            id_g: binds
                                .get_morphism(g)
                                .expect("A morphism could not be inferred"),
                            id_h: binds
                                .get_morphism(h)
                                .expect("A morphism could not be inferred"),
                        },
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}
