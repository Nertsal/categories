use super::*;

impl Rule {
    pub(super) fn new(geng: &Geng, assets: &Rc<Assets>, statement: RuleStatement) -> Self {
        let mut category = Category::new();

        let mut objects = HashMap::new();
        let mut morphisms = HashMap::new();

        fn get_object_or_new(
            category: &mut Category,
            objects: &mut HashMap<String, ObjectId>,
            label: &Label,
            tag: Option<ObjectTag<Option<ObjectId>>>,
            color: Color<f32>,
        ) -> ObjectId {
            let mut new_object = |label: &Label, tag, color| {
                category.new_object(Object::new(
                    label.clone(),
                    tag,
                    util::random_shift(),
                    false,
                    color,
                ))
            };
            match label {
                Label::Name(name) => *objects
                    .entry(name.to_owned())
                    .or_insert_with(|| new_object(label, tag, color)),
                Label::Unknown => new_object(label, tag, color),
            }
        }

        let mut equalities = Equalities::new();

        let mut add_constraints = |constraints: &Constraints, color| -> Vec<CategoryThing> {
            constraints
                .iter()
                .filter_map(|constraint| match constraint {
                    Constraint::RuleObject(label, object) => match object {
                        RuleObject::Object { tag } => {
                            let tag = tag.as_ref().map(|tag| {
                                tag.map_borrowed(|label| match label {
                                    Some(Label::Name(label)) => objects.get(label).copied(),
                                    _ => None,
                                })
                            });
                            Some(CategoryThing::Object {
                                id: get_object_or_new(&mut category, &mut objects, label, tag, color),
                            })
                        }
                        RuleObject::Morphism {
                            constraint: ArrowConstraint { connection, tag },
                        } => {
                            let create = match label {
                                Label::Name(label) => !morphisms.contains_key(label),
                                Label::Unknown => true,
                            };
                            if create {
                                let (connection, object_a, object_b) = match connection {
                                    MorphismConnection::Regular { from, to } => {
                                        let from = get_object_or_new(
                                            &mut category,
                                            &mut objects,
                                            from,
                                            None,
                                            RULE_INFER_COLOR,
                                        );
                                        let to = get_object_or_new(
                                            &mut category,
                                            &mut objects,
                                            to,
                                            None,
                                            RULE_INFER_COLOR,
                                        );
                                        (MorphismConnection::Regular {from, to}, from, to)
                                    },
                                    MorphismConnection::Isomorphism(a, b) => {
                                        let a = get_object_or_new(
                                            &mut category,
                                            &mut objects,
                                            a,
                                            None,
                                            RULE_INFER_COLOR,
                                        );
                                        let b = get_object_or_new(
                                            &mut category,
                                            &mut objects,
                                            b,
                                            None,
                                            RULE_INFER_COLOR,
                                        );
                                        (MorphismConnection::Isomorphism(a, b), a, b)
                                    },
                                };

                                let pos_a = category.objects.get(&object_a).expect("Should have been created if it did not exist").position;
                                let pos_b = category.objects.get(&object_b).expect("Should have been created if it did not exist").position;

                                let tag = tag.as_ref().map(|tag| {
                                    tag.map_borrowed(
                                        |label| match label {
                                            Some(Label::Name(label)) => objects.get(label).copied(),
                                            _ => None,
                                        },
                                        |label| match label {
                                            Some(Label::Name(label)) => {
                                                morphisms.get(label).copied()
                                            }
                                            _ => None,
                                        },
                                    )
                                });

                                let new_morphism = category
                                    .new_morphism(Morphism {
                                        connection,
                                        inner: Arrow::new(label.clone(), tag, color, pos_a, pos_b),
                                    })
                                    .unwrap();

                                match label {
                                    Label::Name(label) => {
                                        morphisms.insert(label.clone(), new_morphism);
                                    }
                                    Label::Unknown => (),
                                }
                                Some(CategoryThing::Morphism { id: new_morphism })
                            } else {
                                None
                            }
                        }
                    },
                    Constraint::MorphismEq(f, g) => {
                        // Check that morphisms exist
                        let check = |label: &Label| {
                            let name = match label {
                                Label::Name(name) => name,
                                Label::Unknown => panic!("An equality must have named labels"),
                            };
                            morphisms.get(name).copied().expect(&format!("An equality expected the morphism {:?} to be constrained explicitly before the equality", name))
                        };

                        let f = check(f);
                        let g = check(g);

                        equalities.insert((f, g));
                        None
                    }
                })
                .collect()
        };

        let mut constructions = statement.iter();
        // Input
        let graph_input = constructions
            .next()
            .map(|construction| match construction {
                RuleConstruction::Forall(constraints) | RuleConstruction::Exists(constraints) => {
                    add_constraints(constraints, RULE_INPUT_COLOR)
                }
            })
            .unwrap_or_default();

        // Middle
        for _ in 1..statement.len().max(1) - 1 {
            let construction = constructions.next().unwrap();
            match construction {
                RuleConstruction::Forall(constraints) => {
                    add_constraints(constraints, RULE_FORALL_COLOR);
                }
                RuleConstruction::Exists(constraints) => {
                    add_constraints(constraints, RULE_EXISTS_COLOR);
                }
            }
        }

        // Output
        let inverse_graph_input = constructions
            .next()
            .map(|construction| match construction {
                RuleConstruction::Forall(constraints) | RuleConstruction::Exists(constraints) => {
                    add_constraints(constraints, RULE_OUTPUT_COLOR)
                }
            })
            .unwrap_or_default();

        Self {
            inverse_statement: invert_statement(&statement).into_iter().last().unwrap(),
            graph: RenderableCategory::new(geng, assets, category, equalities, vec2(1, 1)),
            statement,
            graph_input,
            inverse_graph_input,
        }
    }
}

fn invert_statement(statement: &RuleStatement) -> Vec<RuleStatement> {
    let mut prelude_forall = Vec::new();
    let mut prelude_exists = Vec::new();
    let mut statements = Vec::new();

    let add_object_constraint = |label: Option<&Label>, prelude: &mut Vec<_>| {
        let label = match label {
            Some(Label::Name(label)) => label,
            _ => return,
        };

        let constraints = statement
            .iter()
            .filter_map(|construction| match construction {
                RuleConstruction::Forall(constraints) | RuleConstruction::Exists(constraints) => {
                    constraints.iter().find(|constraint| match constraint {
                        Constraint::RuleObject(Label::Name(name), RuleObject::Object { .. }) => {
                            *name == *label
                        }
                        _ => false,
                    })
                }
            })
            .cloned();
        prelude.extend(constraints);
    };

    let add_morphism_constraint = |label: Option<&Label>, prelude: &mut Vec<_>| {
        let label = match label {
            Some(Label::Name(label)) => label,
            _ => return,
        };

        let constraints = statement
            .iter()
            .filter_map(|construction| match construction {
                RuleConstruction::Forall(constraints) | RuleConstruction::Exists(constraints) => {
                    constraints.iter().find(|constraint| match constraint {
                        Constraint::RuleObject(Label::Name(name), RuleObject::Object { .. }) => {
                            *name == *label
                        }
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
                            Constraint::RuleObject(label, object) => match object {
                                RuleObject::Object { tag } => {
                                    add_object_constraint(Some(label), &mut prelude_forall);

                                    if let Some(tag) = tag {
                                        match tag {
                                            ObjectTag::Initial | ObjectTag::Terminal => (),
                                            ObjectTag::Product(a, b) => {
                                                add_object_constraint(
                                                    a.as_ref(),
                                                    &mut prelude_forall,
                                                );
                                                add_object_constraint(
                                                    b.as_ref(),
                                                    &mut prelude_forall,
                                                );
                                            }
                                        }
                                    }
                                }
                                RuleObject::Morphism { constraint } => {
                                    if let Some(tag) = &constraint.tag {
                                        match tag {
                                            MorphismTag::Identity(a) => {
                                                add_object_constraint(
                                                    a.as_ref(),
                                                    &mut prelude_forall,
                                                );
                                            }
                                            MorphismTag::Composition { first, second } => {
                                                add_morphism_constraint(
                                                    first.as_ref(),
                                                    &mut prelude_forall,
                                                );
                                                add_morphism_constraint(
                                                    second.as_ref(),
                                                    &mut prelude_forall,
                                                );
                                            }
                                            MorphismTag::Unique => (),
                                            MorphismTag::Isomorphism(f, g) => {
                                                add_morphism_constraint(
                                                    f.as_ref(),
                                                    &mut prelude_forall,
                                                );
                                                add_morphism_constraint(
                                                    g.as_ref(),
                                                    &mut prelude_forall,
                                                );
                                            }
                                        }
                                    }
                                }
                            },
                            Constraint::MorphismEq(f, g) => {
                                add_morphism_constraint(Some(f), &mut prelude_forall);
                                add_morphism_constraint(Some(g), &mut prelude_forall);
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

fn invert_constraints(constraints: &Constraints, keep_tags: bool) -> Constraints {
    constraints
        .iter()
        .map(|constraint| match constraint {
            Constraint::RuleObject(label, object) => match object {
                RuleObject::Object { .. } => constraint.clone(),
                RuleObject::Morphism { constraint } => Constraint::RuleObject(
                    label.clone(),
                    RuleObject::Morphism {
                        constraint: ArrowConstraint {
                            tag: if keep_tags {
                                constraint.tag.clone()
                            } else {
                                constraint.tag.as_ref().and_then(|tag| match tag {
                                    MorphismTag::Identity(_) | MorphismTag::Isomorphism(_, _) => {
                                        Some(tag.clone())
                                    }
                                    _ => None,
                                })
                            },
                            ..constraint.clone()
                        },
                    },
                ),
            },
            Constraint::MorphismEq(_, _) => constraint.clone(),
        })
        .collect()
}
