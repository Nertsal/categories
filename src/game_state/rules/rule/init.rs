use super::*;

impl Rule {
    pub(super) fn new(geng: &Geng, assets: &Rc<Assets>, statement: RuleStatement) -> Self {
        let mut graph = Graph::new(default());

        let mut objects = HashMap::new();
        let mut morphisms = HashMap::new();

        fn get_object_or_new(
            graph: &mut Graph,
            objects: &mut HashMap<String, VertexId>,
            label: &RuleLabel,
            tags: Vec<ObjectTag<Option<VertexId>>>,
            color: Color<f32>,
        ) -> VertexId {
            let mut new_object = |label: &RuleLabel, tags, color| {
                graph.graph.new_vertex(ForceVertex {
                    is_anchor: false,
                    body: ForceBody::new(util::random_shift(), POINT_MASS),
                    vertex: Point {
                        label: label.clone(),
                        radius: POINT_RADIUS,
                        tags,
                        color,
                    },
                })
            };
            match label {
                RuleLabel::Name(name) => *objects
                    .entry(name.to_owned())
                    .or_insert_with(|| new_object(label, tags, color)),
                RuleLabel::Any => new_object(label, tags, color),
            }
        }

        let mut add_constraints = |constraints: &Constraints, color| -> Vec<GraphObject> {
            constraints
                .iter()
                .filter_map(|constraint| match constraint {
                    Constraint::RuleObject(label, object) => match object {
                        RuleObject::Vertex { tags } => {
                            let tags = tags
                                .iter()
                                .map(|tag| {
                                    tag.map_borrowed(|label| match label {
                                        Some(RuleLabel::Name(label)) => objects.get(label).copied(),
                                        _ => None,
                                    })
                                })
                                .collect();
                            Some(GraphObject::Vertex {
                                id: get_object_or_new(&mut graph, &mut objects, label, tags, color),
                            })
                        }
                        RuleObject::Edge {
                            constraint: ArrowConstraint { from, to, tags },
                        } => {
                            let create = match label {
                                RuleLabel::Name(label) => !morphisms.contains_key(label),
                                RuleLabel::Any => true,
                            };
                            if create {
                                let from = get_object_or_new(
                                    &mut graph,
                                    &mut objects,
                                    from,
                                    vec![],
                                    RULE_INFER_COLOR,
                                );
                                let to = get_object_or_new(
                                    &mut graph,
                                    &mut objects,
                                    to,
                                    vec![],
                                    RULE_INFER_COLOR,
                                );

                                let tags: Vec<_> = tags
                                    .iter()
                                    .map(|tag| {
                                        tag.map_borrowed(
                                            |label| match label {
                                                Some(RuleLabel::Name(label)) => {
                                                    objects.get(label).copied()
                                                }
                                                _ => None,
                                            },
                                            |label| match label {
                                                Some(RuleLabel::Name(label)) => {
                                                    morphisms.get(label).copied()
                                                }
                                                _ => None,
                                            },
                                        )
                                    })
                                    .collect();

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
                                            tags,
                                            color,
                                        },
                                    ))
                                    .unwrap();

                                match label {
                                    RuleLabel::Name(label) => {
                                        morphisms.insert(label.clone(), new_morphism);
                                    }
                                    RuleLabel::Any => (),
                                }
                                Some(GraphObject::Edge { id: new_morphism })
                            } else {
                                None
                            }
                        }
                    },
                    Constraint::MorphismEq(_, _) => unimplemented!(),
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
            inverse_statement: invert_statement(&statement),
            graph: RenderableGraph::new(geng, assets, graph, vec2(1, 1)),
            statement,
            graph_input,
            inverse_graph_input,
        }
    }
}

fn invert_statement(statement: &RuleStatement) -> RuleStatement {
    statement
        .iter()
        .rev()
        .map(|construction| match construction {
            RuleConstruction::Forall(constraints) => RuleConstruction::Exists(constraints.clone()),
            RuleConstruction::Exists(constraints) => RuleConstruction::Forall(constraints.clone()),
        })
        .collect()
}
