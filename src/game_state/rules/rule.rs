use super::*;

impl GameState {
    /// Attempts to apply a rule.
    /// Returns whether the rule was applied successfully.
    pub fn apply_rule(&mut self, selection: RuleSelection) {
        let rule = self.rules.get_rule(selection.rule()).unwrap();
        rule.apply(&mut self.main_graph, selection.selection())
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

    pub fn forall(mut self, constraints: impl Into<Constraints>) -> Self {
        self.statement
            .push(RuleConstruction::Forall(constraints.into()));
        self
    }

    pub fn exists(mut self, constraints: impl Into<Constraints>) -> Self {
        self.statement
            .push(RuleConstruction::Exists(constraints.into()));
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
            tags: Vec<ObjectTag<VertexId>>,
            color: Color<f32>,
        ) -> VertexId {
            *objects.entry(label.to_owned()).or_insert_with(|| {
                graph.graph.new_vertex(ForceVertex {
                    is_anchor: false,
                    body: ForceBody::new(random_shift(), POINT_MASS),
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
                                            random_shift(),
                                            random_shift(),
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

    fn apply_impl(statement: &[RuleConstruction], bindings: Bindings, graph: &mut Graph) {
        let construction = match statement.first() {
            Some(construction) => construction,
            None => return,
        };

        let statement = &statement[1..];
        match construction {
            RuleConstruction::Forall(constraints) => {
                find_candidates(constraints, &bindings, graph)
                    .map(|candidates| candidates.collect::<Vec<_>>())
                    .map(|candidates| {
                        for mut binds in candidates {
                            binds.extend(bindings.clone());
                            Self::apply_impl(statement, binds, graph);
                        }
                    });
            }
            RuleConstruction::Exists(constraints) => {
                match find_candidates(constraints, &bindings, graph)
                    .map(|mut binds| binds.next())
                    .flatten()
                {
                    Some(mut binds) => {
                        binds.extend(bindings);
                        Self::apply_impl(statement, binds, graph);
                    }
                    None => {
                        apply_constraints(graph, constraints, &bindings);
                    }
                }
            }
        }
    }

    fn apply(&self, graph: &mut Graph, selection: &Vec<GraphObject>) {
        let bindings = match self.statement.first() {
            Some(RuleConstruction::Forall(constraints))
            | Some(RuleConstruction::Exists(constraints)) => {
                match selection_constraints(selection, constraints, graph) {
                    Ok(bindings) => bindings,
                    Err(_) => return,
                }
            }
            _ => Bindings::new(),
        };

        Self::apply_impl(&self.statement, bindings, graph)
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
                    RuleObject::Vertex { tags } => {
                        constraint_object(label, tags, bindings, graph).collect()
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
    tags: &'a Vec<ObjectTag>,
    bindings: &'a Bindings,
    graph: &'a Graph,
) -> impl Iterator<Item = Bindings> + 'a {
    assert!(
        bindings.get_object(label).is_none(),
        "Objects must have unique names!"
    );

    graph.graph.vertices.iter().filter_map(|(&id, vertex)| {
        let mut binds = Bindings::new();
        if tags.iter().all(|constraint| {
            vertex
                .vertex
                .tags
                .iter()
                .any(|tag| match (constraint, tag) {
                    (
                        ObjectTag::Product(constraint0, constraint1),
                        &ObjectTag::Product(object0, object1),
                    ) => {
                        match (
                            bindings.get_object(constraint0),
                            bindings.get_object(constraint1),
                        ) {
                            (Some(constraint0), Some(constraint1)) => {
                                constraint0 == object0 && constraint1 == object1
                                    || constraint0 == object1 && constraint1 == object0
                            }
                            (Some(constraint0), None) => {
                                if constraint0 == object0 {
                                    binds.bind_object(constraint1.to_owned(), object1);
                                    true
                                } else if constraint0 == object1 {
                                    binds.bind_object(constraint1.to_owned(), object0);
                                    true
                                } else {
                                    false
                                }
                            }
                            (None, Some(constraint1)) => {
                                if constraint1 == object0 {
                                    binds.bind_object(constraint0.to_owned(), object1);
                                    true
                                } else if constraint1 == object1 {
                                    binds.bind_object(constraint0.to_owned(), object0);
                                    true
                                } else {
                                    false
                                }
                            }
                            (None, None) => {
                                binds.bind_object(constraint0.to_owned(), object0);
                                binds.bind_object(constraint1.to_owned(), object1);
                                true
                            }
                        }
                    }
                })
        }) {
            binds.bind_object(label.to_owned(), id);
            Some(binds)
        } else {
            None
        }
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

    graph.graph.edges.iter().filter_map(move |(&id, edge)| {
        let mut binds = Bindings::new();
        if check(edge.edge.from, from)
            && check(edge.edge.to, to)
            && constraint.tags.iter().all(|constraint| {
                edge.edge.tags.iter().any(|tag| match (constraint, tag) {
                    (MorphismTag::Unique, MorphismTag::Unique) => true,
                    (MorphismTag::Identity(constraint), &MorphismTag::Identity(object)) => {
                        match bindings.get_object(constraint) {
                            Some(constraint) => constraint == object,
                            None => {
                                binds.bind_object(constraint.to_owned(), object);
                                true
                            }
                        }
                    }
                    (
                        MorphismTag::Composition {
                            first: constraint_first,
                            second: constraint_second,
                        },
                        &MorphismTag::Composition { first, second },
                    ) => {
                        let match_first = match bindings.get_morphism(constraint_first) {
                            Some(constraint) => constraint == first,
                            None => {
                                binds.bind_morphism(constraint_first.to_owned(), first);
                                true
                            }
                        };

                        let match_second = match bindings.get_morphism(constraint_second) {
                            Some(constraint) => constraint == second,
                            None => {
                                binds.bind_morphism(constraint_second.to_owned(), second);
                                true
                            }
                        };

                        match_first && match_second
                    }
                    (
                        MorphismTag::Isomorphism(constraint0, constraint1),
                        &MorphismTag::Isomorphism(morphism0, morphism1),
                    ) => {
                        match (
                            bindings.get_morphism(constraint0),
                            bindings.get_morphism(constraint1),
                        ) {
                            (Some(constraint0), Some(constraint1)) => {
                                constraint0 == morphism0 && constraint1 == morphism1
                                    || constraint0 == morphism1 && constraint1 == morphism0
                            }
                            (Some(constraint0), None) => {
                                if constraint0 == morphism0 {
                                    binds.bind_morphism(constraint1.to_owned(), morphism1);
                                    true
                                } else if constraint0 == morphism1 {
                                    binds.bind_morphism(constraint1.to_owned(), morphism0);
                                    true
                                } else {
                                    false
                                }
                            }
                            (None, Some(constraint1)) => {
                                if constraint1 == morphism0 {
                                    binds.bind_morphism(constraint0.to_owned(), morphism1);
                                    true
                                } else if constraint1 == morphism1 {
                                    binds.bind_morphism(constraint0.to_owned(), morphism0);
                                    true
                                } else {
                                    false
                                }
                            }
                            (None, None) => {
                                binds.bind_morphism(constraint0.to_owned(), morphism0);
                                binds.bind_morphism(constraint1.to_owned(), morphism1);
                                true
                            }
                        }
                    }
                    _ => false,
                })
            })
        {
            binds.bind_morphism(label.to_owned(), id);
            if from.is_none() {
                binds.bind_object(constraint.from.to_owned(), edge.edge.from);
            }
            if to.is_none() {
                binds.bind_object(constraint.to.to_owned(), edge.edge.to);
            }
            Some(binds)
        } else {
            None
        }
    })
}

/// Applies the rule constraints to the graph.
fn apply_constraints(
    graph: &mut Graph,
    constraints: &Constraints,
    bindings: &Bindings,
) -> Vec<GraphAction> {
    let mut bindings = bindings.clone();

    let mut new_vertices = Vec::new();
    let mut new_vertices_names = Vec::new();
    let mut new_edges = Vec::new();
    let mut new_edges_names = Vec::new();

    for constraint in constraints {
        match constraint {
            Constraint::RuleObject(label, rule_object) => match rule_object {
                RuleObject::Vertex { tags } => {
                    let tags: Vec<_> = tags
                        .iter()
                        .map(|tag| tag.map_borrowed(|label| bindings.get_object(label).unwrap()))
                        .collect();
                    let name = tags
                        .iter()
                        .filter_map(|tag| {
                            tag.map_borrowed(|object| {
                                &graph.graph.vertices.get(object).unwrap().vertex.label
                            })
                            .infer_name()
                        })
                        .find(|_| true);
                    new_vertices.push((name, tags));
                    new_vertices_names.push(label.to_owned());
                }
                RuleObject::Edge { constraint } => {
                    let constraint = ArrowConstraint {
                        from: bindings.get_object(&constraint.from).unwrap(),
                        to: bindings.get_object(&constraint.to).unwrap(),
                        tags: constraint
                            .tags
                            .iter()
                            .map(|tag| {
                                tag.map_borrowed(
                                    |label| bindings.get_object(label).unwrap(),
                                    |label| bindings.get_morphism(label).unwrap(),
                                )
                            })
                            .collect(),
                    };
                    let name = constraint
                        .tags
                        .iter()
                        .filter_map(|tag| {
                            tag.map_borrowed(
                                |id| &graph.graph.vertices.get(id).unwrap().vertex.label,
                                |id| &graph.graph.edges.get(id).unwrap().edge.label,
                            )
                            .infer_name()
                        })
                        .find(|_| true);
                    new_edges.push((name, constraint));
                    new_edges_names.push(label.to_owned());
                }
            },
            Constraint::MorphismEq(_, _) => todo!(),
        }
    }

    // Create new vertices
    let mut action_history = Vec::new();
    let action = GameState::graph_action_do(graph, GraphAction::NewVertices(new_vertices));
    // Bind new vertices
    match &action {
        GraphAction::RemoveVertices(vertices) => {
            assert_eq!(vertices.len(), new_vertices_names.len());
            for (label, id) in new_vertices_names.into_iter().zip(vertices.iter().copied()) {
                bindings.bind_object(label, id);
            }
        }
        _ => unreachable!(),
    }
    action_history.push(action);

    // Create new edges
    let action = GameState::graph_action_do(graph, GraphAction::NewEdges(new_edges));
    // Bind new edges
    match &action {
        GraphAction::RemoveEdges(edges) => {
            assert_eq!(edges.len(), new_edges_names.len());
            for (label, id) in new_edges_names.into_iter().zip(edges.iter().copied()) {
                bindings.bind_morphism(label, id);
            }
        }
        _ => unreachable!(),
    }
    action_history.push(action);
    action_history
}
