use super::*;

impl GameState {
    /// Attempts to apply a rule.
    /// Returns whether the rule was applied successfully.
    pub fn apply_rule(&mut self, selection: RuleSelection) -> bool {
        let rule = self.rules.get_rule(selection.rule()).unwrap();
        match rule.action(&mut self.main_graph, selection.selection()) {
            Ok(actions) => {
                for action in actions {
                    self.action_do(action);
                }
                true
            }
            Err(_) => false,
        }
    }
}

pub type RuleStatement = Vec<RuleConstruction>;

pub struct RuleBuilder {
    statement: RuleStatement,
}

impl RuleBuilder {
    pub fn new() -> Self {
        Self { statement: vec![] }
    }

    pub fn forall(mut self, constraints: Constraints) -> Self {
        self.statement.push(RuleConstruction::Forall(constraints));
        self
    }

    pub fn exists(mut self, constraints: Constraints) -> Self {
        self.statement.push(RuleConstruction::Exists(constraints));
        self
    }

    pub fn such_that_forall(mut self, constraints: Constraints) -> Self {
        self.statement.push(RuleConstruction::SuchThat);
        self.statement.push(RuleConstruction::Forall(constraints));
        self
    }

    pub fn such_that_exists(mut self, constraints: Constraints) -> Self {
        self.statement.push(RuleConstruction::SuchThat);
        self.statement.push(RuleConstruction::Exists(constraints));
        self
    }

    pub fn build(self) -> Rule {
        Rule::new(self.statement)
    }
}

pub type Label = String;

#[derive(Debug, Default, Clone)]
pub struct Bindings {
    objects: HashMap<Label, VertexId>,
    morphisms: HashMap<Label, EdgeId>,
}

impl Bindings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extend(&mut self, bindings: Self) {
        self.objects.extend(bindings.objects.into_iter());
        self.morphisms.extend(bindings.morphisms.into_iter());
    }

    pub fn bind_object(&mut self, label: Label, id: VertexId) -> Option<VertexId> {
        self.objects.insert(label, id)
    }

    pub fn bind_morphism(&mut self, label: Label, id: EdgeId) -> Option<EdgeId> {
        self.morphisms.insert(label, id)
    }

    pub fn get_object(&self, label: &str) -> Option<VertexId> {
        self.objects.get(label).copied()
    }

    pub fn get_morphism(&self, label: &str) -> Option<EdgeId> {
        self.morphisms.get(label).copied()
    }
}

pub struct Rule {
    statement: RuleStatement,
    graph: Graph,
    graph_input: Vec<GraphObject>,
}

impl Rule {
    fn new(statement: RuleStatement) -> Self {
        let mut graph = Graph::new(default());

        let mut objects = HashMap::new();
        let mut morphisms = HashMap::new();

        fn get_object_or_new(
            graph: &mut Graph,
            objects: &mut HashMap<Label, VertexId>,
            label: &str,
        ) -> VertexId {
            *objects.entry(label.to_owned()).or_insert_with(|| {
                graph.graph.new_vertex(ForceVertex {
                    is_anchor: false,
                    body: ForceBody::new(random_shift(), POINT_MASS),
                    vertex: Point {
                        label: label.to_owned(),
                        radius: POINT_RADIUS,
                        color: Color::WHITE,
                    },
                })
            })
        }

        let mut add_constraints = |constraints: &Constraints| {
            for constraint in constraints {
                match constraint {
                    Constraint::RuleObject(label, object) => {
                        match object {
                            RuleObject::Vertex { .. } => {
                                get_object_or_new(&mut graph, &mut objects, label);
                            }
                            RuleObject::Edge { constraint } => {
                                if !morphisms.contains_key(label) {
                                    let from = get_object_or_new(
                                        &mut graph,
                                        &mut objects,
                                        &constraint.from,
                                    );
                                    let to =
                                        get_object_or_new(&mut graph, &mut objects, &constraint.to);

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
                                            random_shift(),
                                            random_shift(),
                                            ARROW_BODIES,
                                            ARROW_MASS,
                                            Arrow::new(label, from, to, tags, Color::WHITE),
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

        for construction in &statement {
            match construction {
                RuleConstruction::Forall(constraints) | RuleConstruction::Exists(constraints) => {
                    add_constraints(constraints)
                }
                RuleConstruction::SuchThat => continue,
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
                RuleConstruction::SuchThat => (),
            }
        }

        Self {
            statement,
            graph,
            graph_input,
        }
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    pub fn graph_mut(&mut self) -> &mut Graph {
        &mut self.graph
    }

    pub fn update_graph(&mut self, delta_time: f32) {
        self.graph.update(delta_time);
    }

    pub fn statement(&self) -> &RuleStatement {
        &self.statement
    }

    pub fn graph_input(&self) -> &Vec<GraphObject> {
        &self.graph_input
    }

    fn apply(
        statement: &[RuleConstruction],
        bindings: Bindings,
        graph: &Graph,
    ) -> Vec<GraphActionDo> {
        if let Some(construction) = statement.first() {
            let statement = &statement[1..];
            match construction {
                RuleConstruction::SuchThat => Self::apply(statement, bindings, graph),
                RuleConstruction::Forall(constraints) => {
                    find_candidates(constraints, &bindings, graph)
                        .map(|candidates| {
                            candidates
                                .flat_map(|mut binds| {
                                    binds.extend(bindings.clone());
                                    Self::apply(statement, binds, graph)
                                })
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default()
                }
                RuleConstruction::Exists(constraints) => {
                    match find_candidates(constraints, &bindings, graph)
                        .map(|mut binds| binds.next())
                        .flatten()
                    {
                        Some(mut binds) => {
                            binds.extend(bindings);
                            Self::apply(statement, binds, graph)
                        }
                        None => apply_constraints(constraints, &bindings, graph),
                    }
                }
            }
        } else {
            vec![]
        }
    }

    fn action(
        &self,
        graph: &Graph,
        selection: &Vec<GraphObject>,
    ) -> Result<Vec<GraphActionDo>, ()> {
        let bindings = match self.statement.first() {
            Some(RuleConstruction::Forall(constraints))
            | Some(RuleConstruction::Exists(constraints)) => {
                selection_constraints(selection, constraints, graph)?
            }
            _ => Bindings::new(),
        };

        Ok(Self::apply(&self.statement, bindings, graph))
    }
}

fn selection_constraints(
    selection: &Vec<GraphObject>,
    constraints: &Constraints,
    graph: &Graph,
) -> Result<Bindings, ()> {
    let mut selection = selection.iter();
    let mut bindings = Bindings::new();

    fn bind_object(bindings: &mut Bindings, label: &str, constraint: VertexId) -> bool {
        match bindings.get_object(label) {
            Some(object) => object == constraint,
            None => {
                bindings.bind_object(label.to_owned(), constraint);
                true
            }
        }
    }

    for constraint in constraints {
        match constraint {
            Constraint::RuleObject(label, object) => match object {
                RuleObject::Vertex { .. } => match selection.next() {
                    Some(GraphObject::Vertex { id }) => {
                        if bindings.bind_object(label.to_owned(), *id).is_some() {
                            return Err(());
                        }
                    }
                    _ => return Err(()),
                },
                RuleObject::Edge { constraint } => match selection.next() {
                    Some(GraphObject::Edge { id }) => {
                        let edge = graph.graph.edges.get(id).unwrap();
                        if !bind_object(&mut bindings, &constraint.from, edge.edge.from)
                            || !bind_object(&mut bindings, &constraint.to, edge.edge.to)
                        {
                            return Err(());
                        }

                        bindings.bind_morphism(label.to_owned(), *id);
                    }
                    _ => return Err(()),
                },
            },
            Constraint::MorphismEq(_, _) => todo!(),
        }
    }

    Ok(bindings)
}

pub fn find_candidates<'a>(
    constraints: &'a [Constraint],
    bindings: &'a Bindings,
    graph: &'a Graph,
) -> Option<impl Iterator<Item = Bindings> + 'a> {
    let constraint = match constraints.first() {
        Some(constraint) => constraint,
        None => return None,
    };
    let constraints = &constraints[1..];

    let binds: Vec<_> = match constraint {
        Constraint::RuleObject(label, object) => {
            if bindings.get_object(label).is_some() || bindings.get_morphism(label).is_some() {
                vec![Bindings::new()]
            } else {
                match object {
                    RuleObject::Vertex { .. } => {
                        constraint_object(label, bindings, graph).collect()
                    }
                    RuleObject::Edge { constraint } => {
                        constraint_morphism(label, constraint, bindings, graph).collect()
                    }
                }
            }
        }
        Constraint::MorphismEq(_, _) => unimplemented!(),
    };

    Some(binds.into_iter().flat_map(|binds| {
        let mut old_binds = binds.clone();
        old_binds.extend(bindings.clone());
        let binds = match find_candidates(constraints, &old_binds, graph) {
            Some(new_binds) => new_binds
                .map(move |mut next_binds| {
                    next_binds.extend(binds.clone());
                    next_binds
                })
                .collect::<Vec<_>>(),
            None => vec![binds],
        };
        binds
    }))
}

fn constraint_object<'a>(
    label: &'a Label,
    bindings: &'a Bindings,
    graph: &'a Graph,
) -> impl Iterator<Item = Bindings> + 'a {
    assert!(
        bindings.get_object(label).is_none(),
        "Objects must have unique names!"
    );

    graph.graph.vertices.iter().map(|(&id, _)| {
        let mut binds = Bindings::new();
        binds.bind_object(label.to_owned(), id);
        binds
    })
}

fn constraint_morphism<'a>(
    label: &'a Label,
    constraint: &'a ArrowConstraint,
    bindings: &'a Bindings,
    graph: &'a Graph,
) -> impl Iterator<Item = Bindings> + 'a {
    assert!(
        bindings.get_morphism(label).is_none(),
        "Morphisms must have unique names!"
    );

    let from = bindings.get_object(&constraint.from);
    let to = bindings.get_object(&constraint.to);

    fn check<T: Eq>(value: T, constraint: Option<T>) -> bool {
        match constraint {
            None => true,
            Some(constraint) => value == constraint,
        }
    }

    graph
        .graph
        .edges
        .iter()
        .filter(move |(_, edge)| check(edge.edge.from, from) && check(edge.edge.to, to))
        .map(move |(&id, edge)| {
            let mut binds = Bindings::new();
            binds.bind_morphism(label.to_owned(), id);

            if from.is_none() {
                binds.bind_object(constraint.from.to_owned(), edge.edge.from);
            }
            if to.is_none() {
                binds.bind_object(constraint.to.to_owned(), edge.edge.to);
            }

            binds
        })
}

fn apply_constraints(
    constraints: &Constraints,
    bindings: &Bindings,
    graph: &Graph,
) -> Vec<GraphActionDo> {
    let input_vertices: Vec<_> = bindings.objects.values().copied().collect();
    let input_edges: Vec<_> = bindings.morphisms.values().copied().collect();
    let mut new_vertices = HashMap::new();

    let mut new_vertices_count = 0;
    let mut new_edges = Vec::new();

    let find_object = |label,
                       input_vertices: &Vec<VertexId>,
                       new_vertices: &HashMap<Label, usize>|
     -> Option<usize> {
        bindings
            .get_object(label)
            .and_then(|id| input_vertices.iter().position(|&object| object == id))
            .or_else(|| new_vertices.get(label).copied())
    };

    let find_morphism = |label, input_vertices: &Vec<VertexId>| -> Option<usize> {
        bindings
            .get_object(label)
            .and_then(|id| input_vertices.iter().position(|&object| object == id))
    };

    for constraint in constraints {
        match constraint {
            Constraint::RuleObject(label, object) => match object {
                RuleObject::Vertex { .. } => {
                    new_vertices
                        .insert(label.to_owned(), input_vertices.len() + new_vertices_count);
                    new_vertices_count += 1;
                }
                RuleObject::Edge { constraint } => {
                    let from =
                        find_object(&constraint.from, &input_vertices, &new_vertices).unwrap();
                    let to = find_object(&constraint.to, &input_vertices, &new_vertices).unwrap();
                    let new_edge = ArrowConstraint::new(
                        from,
                        to,
                        constraint
                            .tags
                            .iter()
                            .map(|tag| {
                                tag.map_borrowed(
                                    |label| {
                                        bindings
                                            .get_object(label)
                                            .and_then(|id| {
                                                input_vertices
                                                    .iter()
                                                    .position(|&object| object == id)
                                            })
                                            .unwrap()
                                    },
                                    |label| {
                                        bindings
                                            .get_morphism(label)
                                            .and_then(|id| {
                                                input_edges.iter().position(|&object| object == id)
                                            })
                                            .unwrap()
                                    },
                                )
                            })
                            .collect(),
                    );
                    new_edges.push(new_edge);
                }
            },
            Constraint::MorphismEq(_, _) => todo!(),
        }
    }

    vec![GraphActionDo::ApplyRule {
        input_vertices,
        input_edges,
        new_vertices: new_vertices_count,
        new_edges,
        remove_vertices: vec![],
        remove_edges: vec![],
    }]
}
