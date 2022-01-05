use super::*;

impl Rule {
    pub(super) fn new(geng: &Geng, assets: &Rc<Assets>, statement: RuleStatement) -> Self {
        let mut graph = Graph::new(default());

        let mut objects = HashMap::new();
        let mut morphisms = HashMap::new();

        fn get_object_or_new(
            graph: &mut Graph,
            objects: &mut HashMap<String, VertexId>,
            label: &Label,
            tag: Option<ObjectTag<Option<VertexId>>>,
            color: Color<f32>,
        ) -> VertexId {
            let mut new_object = |label: &Label, tag, color| {
                graph.graph.new_vertex(ForceVertex {
                    is_anchor: false,
                    body: ForceBody::new(util::random_shift(), POINT_MASS),
                    vertex: Point {
                        label: label.clone(),
                        radius: POINT_RADIUS,
                        tag,
                        color,
                    },
                })
            };
            match label {
                Label::Name(name) => *objects
                    .entry(name.to_owned())
                    .or_insert_with(|| new_object(label, tag, color)),
                Label::Any => new_object(label, tag, color),
            }
        }

        let mut add_constraints = |constraints: &Constraints, color| -> Vec<GraphObject> {
            constraints
                .iter()
                .filter_map(|constraint| match constraint {
                    Constraint::RuleObject(label, object) => match object {
                        RuleObject::Vertex { tag } => {
                            let tag = tag.as_ref().map(|tag| {
                                tag.map_borrowed(|label| match label {
                                    Some(Label::Name(label)) => objects.get(label).copied(),
                                    _ => None,
                                })
                            });
                            Some(GraphObject::Vertex {
                                id: get_object_or_new(&mut graph, &mut objects, label, tag, color),
                            })
                        }
                        RuleObject::Edge {
                            constraint: ArrowConstraint { from, to, tag },
                        } => {
                            let create = match label {
                                Label::Name(label) => !morphisms.contains_key(label),
                                Label::Any => true,
                            };
                            if create {
                                let from = get_object_or_new(
                                    &mut graph,
                                    &mut objects,
                                    from,
                                    None,
                                    RULE_INFER_COLOR,
                                );
                                let to = get_object_or_new(
                                    &mut graph,
                                    &mut objects,
                                    to,
                                    None,
                                    RULE_INFER_COLOR,
                                );

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

                                let new_morphism = graph
                                    .graph
                                    .new_edge(ForceEdge::new(
                                        util::random_shift(),
                                        util::random_shift(),
                                        ARROW_BODIES,
                                        ARROW_MASS,
                                        Arrow {
                                            label: label.clone(),
                                            from,
                                            to,
                                            tag,
                                            color,
                                        },
                                    ))
                                    .unwrap();

                                match label {
                                    Label::Name(label) => {
                                        morphisms.insert(label.clone(), new_morphism);
                                    }
                                    Label::Any => (),
                                }
                                Some(GraphObject::Edge { id: new_morphism })
                            } else {
                                None
                            }
                        }
                    },
                    Constraint::MorphismEq(f, g) => {
                        // Check that morphisms exist
                        for morphism in vec![f, g] {
                            let name = match morphism {
                                Label::Name(name) => name,
                                Label::Any => panic!("An equality must have named labels"),
                            };

                            if !morphisms.contains_key(name) && !constraints.iter().any(|constraint| match constraint {
                                Constraint::RuleObject(
                                    Label::Name(label),
                                    RuleObject::Edge { .. },
                                ) if *name == *label => true,
                                _ => false,
                            }) {
                                panic!("An equality expected the morphism {:?} to be constrained explicitly", name);
                            }
                        }

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
            graph: RenderableGraph::new(geng, assets, graph, vec2(1, 1)),
            statement,
            graph_input,
            inverse_graph_input,
        }
    }
}

fn invert_statement(statement: &RuleStatement) -> Vec<RuleStatement> {
    let mut prelude = Vec::new();
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
                        Constraint::RuleObject(Label::Name(name), RuleObject::Vertex { .. }) => {
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
                        Constraint::RuleObject(Label::Name(name), RuleObject::Vertex { .. }) => {
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
                    prelude.extend(forall);
                }
                last_forall = Some(constraints.clone());
            }
            RuleConstruction::Exists(constraints) => {
                if let Some(forall) = last_forall.take() {
                    // Constraint used objects
                    for constraint in constraints.iter().chain(forall.iter()) {
                        match constraint {
                            Constraint::RuleObject(label, object) => match object {
                                RuleObject::Vertex { tag } => {
                                    add_object_constraint(Some(label), &mut prelude);

                                    if let Some(tag) = tag {
                                        match tag {
                                            ObjectTag::Product(a, b) => {
                                                add_object_constraint(a.as_ref(), &mut prelude);
                                                add_object_constraint(b.as_ref(), &mut prelude);
                                            }
                                        }
                                    }
                                }
                                RuleObject::Edge { constraint } => {
                                    if let Some(tag) = &constraint.tag {
                                        match tag {
                                            MorphismTag::Identity(a) => {
                                                add_object_constraint(a.as_ref(), &mut prelude);
                                            }
                                            MorphismTag::Composition { first, second } => {
                                                add_morphism_constraint(
                                                    first.as_ref(),
                                                    &mut prelude,
                                                );
                                                add_morphism_constraint(
                                                    second.as_ref(),
                                                    &mut prelude,
                                                );
                                            }
                                            MorphismTag::Unique => (),
                                            MorphismTag::Isomorphism(f, g) => {
                                                add_morphism_constraint(f.as_ref(), &mut prelude);
                                                add_morphism_constraint(g.as_ref(), &mut prelude);
                                            }
                                        }
                                    }
                                }
                            },
                            Constraint::MorphismEq(f, g) => {
                                add_morphism_constraint(Some(f), &mut prelude);
                                add_morphism_constraint(Some(g), &mut prelude);
                            }
                        }
                    }

                    // Construct an inverse rule
                    let inv_forall = invert_constraints(constraints);
                    let inv_exists = invert_constraints(&forall);
                    statements.push(vec![
                        RuleConstruction::Forall(inv_forall),
                        RuleConstruction::Forall(prelude.clone()),
                        RuleConstruction::Exists(inv_exists),
                    ]);
                    prelude.extend(forall);
                }
            }
        };
    }

    statements
}

fn invert_constraints(constraints: &Constraints) -> Constraints {
    constraints
        .iter()
        .map(|constraint| match constraint {
            Constraint::RuleObject(label, object) => match object {
                RuleObject::Vertex { .. } => constraint.clone(),
                RuleObject::Edge { constraint } => Constraint::RuleObject(
                    label.clone(),
                    RuleObject::Edge {
                        constraint: ArrowConstraint {
                            tag: constraint.tag.as_ref().and_then(|tag| match tag {
                                MorphismTag::Identity(_) | MorphismTag::Isomorphism(_, _) => {
                                    Some(tag.clone())
                                }
                                _ => None,
                            }),
                            ..constraint.clone()
                        },
                    },
                ),
            },
            Constraint::MorphismEq(_, _) => constraint.clone(),
        })
        .collect()
}
