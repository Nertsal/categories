use super::*;

impl Rule {
    pub(super) fn new(statement: RuleStatement) -> Self {
        let mut graph = Graph::new(default());

        let mut objects = HashMap::new();
        let mut morphisms = HashMap::new();

        fn get_object_or_new(
            graph: &mut Graph,
            objects: &mut HashMap<Label, VertexId>,
            label: &str,
            tags: Vec<ObjectTag<VertexId>>,
            color: Color<f32>,
        ) -> VertexId {
            *objects.entry(label.to_owned()).or_insert_with(|| {
                graph.graph.new_vertex(ForceVertex {
                    is_anchor: false,
                    body: ForceBody::new(util::random_shift(), POINT_MASS),
                    vertex: Point {
                        label: label.to_owned(),
                        radius: POINT_RADIUS,
                        tags,
                        color,
                    },
                })
            })
        }

        let mut add_constraints = |constraints: &Constraints, color: Color<f32>| {
            for constraint in constraints {
                match constraint {
                    Constraint::RuleObject(label, object) => {
                        match object {
                            RuleObject::Vertex { tags } => {
                                let tags = tags
                                    .iter()
                                    .map(|tag| {
                                        tag.map_borrowed(|object| *objects.get(object).unwrap())
                                    })
                                    .collect();
                                get_object_or_new(&mut graph, &mut objects, label, tags, color);
                            }
                            RuleObject::Edge { constraint } => {
                                if !morphisms.contains_key(label) {
                                    let from = get_object_or_new(
                                        &mut graph,
                                        &mut objects,
                                        &constraint.from,
                                        vec![],
                                        RULE_INFER_COLOR,
                                    );
                                    let to = get_object_or_new(
                                        &mut graph,
                                        &mut objects,
                                        &constraint.to,
                                        vec![],
                                        RULE_INFER_COLOR,
                                    );

                                    let tags: Vec<_> = constraint
                                    .tags
                                    .iter()
                                    .map(|tag| tag.map_borrowed(
                                        |object| *objects.get(object).expect("Objects in tags must be created explicitly"), 
                                        |morphism| *morphisms.get(morphism).expect("Morphisms in tags must be created explicitly"), ))
                                    .collect();

                                    let new_morphism = graph
                                        .graph
                                        .new_edge(ForceEdge::new(
                                            util::random_shift(),
                                            util::random_shift(),
                                            ARROW_BODIES,
                                            ARROW_MASS,
                                            Arrow::new(label, from, to, tags, color),
                                        ))
                                        .unwrap();
                                    morphisms.insert(label.to_owned(), new_morphism);
                                }
                            }
                        }
                    }
                    Constraint::MorphismEq(_, _) => unimplemented!(),
                }
            }
        };

        let mut constructions = statement.iter();
        // Input
        if let Some(construction) = constructions.next() {
            match construction {
                RuleConstruction::Forall(constraints) | RuleConstruction::Exists(constraints) => {
                    add_constraints(constraints, RULE_INPUT_COLOR)
                }
            }
        }
        // Middle
        for _ in 1..statement.len().max(1) - 1 {
            let construction = constructions.next().unwrap();
            match construction {
                RuleConstruction::Forall(constraints) => {
                    add_constraints(constraints, RULE_FORALL_COLOR)
                }
                RuleConstruction::Exists(constraints) => {
                    add_constraints(constraints, RULE_EXISTS_COLOR)
                }
            }
        }
        // Output
        if let Some(construction) = constructions.next() {
            match construction {
                RuleConstruction::Forall(constraints) | RuleConstruction::Exists(constraints) => {
                    add_constraints(constraints, RULE_OUTPUT_COLOR)
                }
            }
        }

        let mut graph_input = Vec::new();
        if let Some(construction) = statement.first() {
            match construction {
                RuleConstruction::Forall(constraints) | RuleConstruction::Exists(constraints) => {
                    for constraint in constraints {
                        match constraint {
                            Constraint::RuleObject(label, object) => match object {
                                RuleObject::Vertex { .. } => {
                                    let id = *objects.get(label).unwrap();
                                    graph_input.push(GraphObject::Vertex { id });
                                }
                                RuleObject::Edge { .. } => {
                                    let id = *morphisms.get(label).unwrap();
                                    graph_input.push(GraphObject::Edge { id });
                                }
                            },
                            Constraint::MorphismEq(_, _) => continue,
                        }
                    }
                }
            }
        }

        Self {
            statement,
            graph,
            graph_input,
        }
    }
}
