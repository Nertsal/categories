use super::*;

mod geom;
mod select;

use category::Constraint;
use geom::*;
pub use select::*;

#[derive(Debug)]
pub struct RuleSelection {
    rule_index: usize,
    current_selection: Option<RuleInput<Label>>,
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
        let mut selection = RuleSelection {
            selected: Bindings::new(),
            inferred_options: None,
            current_selection: None,
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
        self.current_selection.as_ref()
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
        if self.current_selection.is_none() {
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

        self.infer_current(category, rules);
        self.current()
    }

    pub fn inferred_options(&self) -> &Option<Vec<RuleInput<Label>>> {
        &self.inferred_options
    }

    /// Infer possible selections for the current rule selection
    fn infer_current(&mut self, category: &Category, rules: &Vec<RenderableRule>) {
        self.inferred_options = rules.get(self.rule()).and_then(|rule| {
            let statement = match self.inverse {
                None => rule.inner.get_statement(),
                Some(inverse) => rule.inverse[inverse].get_statement(),
            };

            let mut constraints = statement
                .iter()
                .map_while(|construction| match construction {
                    category::RuleConstruction::Forall(c) => Some(c),
                    category::RuleConstruction::Exists(_) => None,
                });

            constraints.next().map(|construction| {
                let constraints = construction
                    .iter()
                    .chain(constraints.flat_map(|x| x))
                    .cloned()
                    .collect();

                let bindings = &rule.bindings;

                let check_morphism = |label| match self.selected.get_morphism(label) {
                    None => bindings.get_morphism(label).map(|id| RuleInput::Morphism {
                        label: label.clone(),
                        id,
                    }),
                    Some(_) => None,
                };

                let current = construction.iter().find_map(|constraint| match constraint {
                    Constraint::Object { label, .. } => match self.selected.get_object(label) {
                        Some(_) => None,
                        None => bindings.get_object(label).map(|id| RuleInput::Object {
                            label: label.clone(),
                            id,
                        }),
                    },
                    Constraint::Morphism { label, .. } => check_morphism(label),
                    Constraint::Equality(f, g) => check_morphism(f).or_else(|| check_morphism(g)),
                    Constraint::Commute { f, g, h } => check_morphism(f)
                        .or_else(|| check_morphism(g))
                        .or_else(|| check_morphism(h)),
                });

                let inferred = current
                    .as_ref()
                    .map(|constraint| {
                        infer_construction(constraint, &constraints, category, &self.selected)
                    })
                    .unwrap_or_default();
                self.current_selection = current;
                inferred
            })
        });
    }
}

fn infer_construction(
    input_constraint: &RuleInput<Label>,
    all_constraints: &Constraints,
    category: &Category,
    bindings: &Bindings,
) -> Vec<RuleInput<Label>> {
    category
        .find_candidates(all_constraints, bindings)
        .map(|candidates| {
            candidates
                .into_iter()
                .map(|mut binds| {
                    binds.extend(bindings.clone());
                    match input_constraint {
                        RuleInput::Object { label, .. } => RuleInput::Object {
                            label: label.clone(),
                            id: binds
                                .get_object(label)
                                .expect("An object could not be inferred"),
                        },
                        RuleInput::Morphism { label, .. } => RuleInput::Morphism {
                            label: label.clone(),
                            id: binds
                                .get_morphism(label)
                                .expect("A morphism could not be inferred"),
                        },
                        RuleInput::Equality {
                            label_f, label_g, ..
                        } => RuleInput::Equality {
                            label_f: label_f.clone(),
                            label_g: label_g.clone(),
                            id_f: binds
                                .get_morphism(label_f)
                                .expect("A morphism could not be inferred"),
                            id_g: binds
                                .get_morphism(label_g)
                                .expect("A morphism could not be inferred"),
                        },
                        RuleInput::Commute {
                            label_f,
                            label_g,
                            label_h,
                            ..
                        } => RuleInput::Commute {
                            label_f: label_f.clone(),
                            label_g: label_g.clone(),
                            label_h: label_h.clone(),
                            id_f: binds
                                .get_morphism(label_f)
                                .expect("A morphism could not be inferred"),
                            id_g: binds
                                .get_morphism(label_g)
                                .expect("A morphism could not be inferred"),
                            id_h: binds
                                .get_morphism(label_h)
                                .expect("A morphism could not be inferred"),
                        },
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}
