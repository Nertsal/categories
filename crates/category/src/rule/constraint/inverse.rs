use super::*;

impl<L: Label> Rule<L> {
    pub fn invert(&self) -> Vec<Self> {
        invert_statement(&self.statement)
            .into_iter()
            .map(|statement| Self { statement })
            .collect()
    }
}

fn invert_statement<L: Label>(statement: &RuleStatement<L>) -> Vec<RuleStatement<L>> {
    let mut prelude_forall = Vec::new();
    let mut prelude_exists = Vec::new();
    let mut statements = Vec::new();

    let add_object_constraint = |label: &L, prelude: &mut Vec<_>| {
        let constraints = statement
            .iter()
            .filter_map(|construction| match construction {
                RuleConstruction::Forall(constraints) | RuleConstruction::Exists(constraints) => {
                    constraints.iter().find(|constraint| match constraint {
                        Constraint::Object { label: name, .. } => *name == *label,
                        _ => false,
                    })
                }
            })
            .cloned();
        prelude.extend(constraints);
    };

    let add_morphism_constraint = |label: &L, prelude: &mut Vec<_>| {
        let constraints = statement
            .iter()
            .filter_map(|construction| match construction {
                RuleConstruction::Forall(constraints) | RuleConstruction::Exists(constraints) => {
                    constraints.iter().find(|constraint| match constraint {
                        Constraint::Morphism { label: name, .. } => *name == *label,
                        _ => false,
                    })
                }
            })
            .cloned();
        prelude.extend(constraints);
    };

    let mut last_forall = None;

    for construction in statement {
        match construction {
            RuleConstruction::Forall(constraints) => {
                if let Some(forall) = last_forall.take() {
                    prelude_exists.extend(forall);
                }
                last_forall = Some(constraints.clone());
            }
            RuleConstruction::Exists(constraints) => {
                if let Some(forall) = last_forall.take() {
                    // Constraint used objects
                    for constraint in constraints.iter().chain(forall.iter()) {
                        match constraint {
                            Constraint::Object { label, tags } => {
                                add_object_constraint(label, &mut prelude_forall);

                                for tag in tags {
                                    match tag {
                                        ObjectTag::Initial | ObjectTag::Terminal => (),
                                        ObjectTag::Product(a, b) => {
                                            add_object_constraint(a, &mut prelude_forall);
                                            add_object_constraint(b, &mut prelude_forall);
                                        }
                                    }
                                }
                            }
                            Constraint::Morphism { tags, .. } => {
                                for tag in tags {
                                    match tag {
                                        MorphismTag::Identity(a) => {
                                            add_object_constraint(a, &mut prelude_forall);
                                        }
                                        MorphismTag::Composition { first, second } => {
                                            add_morphism_constraint(first, &mut prelude_forall);
                                            add_morphism_constraint(second, &mut prelude_forall);
                                        }
                                        MorphismTag::Unique => (),
                                        MorphismTag::Isomorphism(f, g) => {
                                            add_morphism_constraint(f, &mut prelude_forall);
                                            add_morphism_constraint(g, &mut prelude_forall);
                                        }
                                    }
                                }
                            }
                            Constraint::Equality(f, g) => {
                                add_morphism_constraint(f, &mut prelude_forall);
                                add_morphism_constraint(g, &mut prelude_forall);
                            }
                            Constraint::Commute { f, g, h } => {
                                add_morphism_constraint(f, &mut prelude_forall);
                                add_morphism_constraint(g, &mut prelude_forall);
                                add_morphism_constraint(h, &mut prelude_forall);
                            }
                        }
                    }

                    // Construct an inverse rule
                    let inv_forall = invert_constraints(constraints, false);
                    let inv_exists = invert_constraints(&forall, true);

                    let mut statement = Vec::new();
                    statement.push(RuleConstruction::Forall(inv_forall));
                    if !prelude_forall.is_empty() {
                        statement.push(RuleConstruction::Forall(prelude_forall.clone()));
                    }
                    if !prelude_exists.is_empty() {
                        statement.push(RuleConstruction::Exists(prelude_exists.clone()));
                    }
                    statement.push(RuleConstruction::Exists(inv_exists));

                    statements.push(statement);
                    prelude_forall.extend(forall);
                }
            }
        };
    }

    statements
}

fn invert_constraints<L: Label>(constraints: &Constraints<L>, keep_tags: bool) -> Constraints<L> {
    constraints
        .iter()
        .map(|constraint| match constraint {
            Constraint::Object { .. } => constraint.clone(),
            Constraint::Morphism {
                label,
                connection,
                tags,
            } => Constraint::Morphism {
                label: label.clone(),
                connection: connection.clone(),
                tags: if keep_tags {
                    tags.clone()
                } else {
                    tags.iter()
                        .filter_map(|tag| match tag {
                            MorphismTag::Identity(_) => Some(tag.clone()),
                            _ => None,
                        })
                        .collect()
                },
            },
            Constraint::Equality(_, _) => constraint.clone(),
            Constraint::Commute { .. } => constraint.clone(),
        })
        .collect()
}
