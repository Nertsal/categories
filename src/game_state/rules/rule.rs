use super::*;

impl GameState {
    /// Attempts to apply a rule.
    /// Returns whether the rule was applied successfully.
    pub fn apply_rule(&mut self, selection: RuleSelection) -> bool {
        let rule = self.rules.get_rule(selection.rule()).unwrap();
        if !rule.check_input(&self.main_graph, selection.selection()) {
            return false;
        }

        match rule.action(&mut self.main_graph, selection.selection()) {
            Ok(action) => {
                self.action_do(action);
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

    pub fn forall(mut self) -> Self {
        self.statement.push(RuleConstruction::Forall());
        self
    }

    pub fn exists(mut self) -> Self {
        self.statement.push(RuleConstruction::Exists());
        self
    }

    pub fn such_that_forall(mut self) -> Self {
        self.statement.push(RuleConstruction::SuchThat);
        self.statement.push(RuleConstruction::Forall());
        self
    }

    pub fn such_that_exists(mut self) -> Self {
        self.statement.push(RuleConstruction::SuchThat);
        self.statement.push(RuleConstruction::Exists());
        self
    }

    pub fn build(self) -> Rule {
        Rule {
            statement: self.statement,
        }
    }
}

pub type Label = String;

#[derive(Default, Clone)]
struct Bindings {
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

    pub fn get_object(&self, label: &Label) -> Option<VertexId> {
        self.objects.get(label).copied()
    }

    pub fn get_morphism(&self, label: &Label) -> Option<EdgeId> {
        self.morphisms.get(label).copied()
    }
}

pub struct Rule {
    statement: RuleStatement,
}

impl Rule {
    fn new(statement: RuleStatement) -> Self {
        Self { statement }
    }

    fn apply<'a>(
        mut statement: impl Iterator<Item = &'a RuleConstruction>,
        bindings: Bindings,
        graph: &Graph,
    ) -> Vec<GraphActionDo> {
        if let Some(construction) = statement.next() {
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
                        .map(|binds| binds.next())
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

    fn check_input(&self, graph: &Graph, selection: &Vec<GraphObject>) -> bool {
        todo!()
    }

    fn action(&self, graph: &Graph, selection: &Vec<GraphObject>) -> Result<GraphActionDo, ()> {
        todo!()
    }
}

fn find_candidates<'a>(
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
        Constraint::RuleObject(label, object) => match object {
            RuleObject::Vertex => constraint_object(label, bindings, graph).collect(),
            RuleObject::Edge { constraint } => {
                constraint_morphism(label, constraint, bindings, graph).collect()
            }
        },
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
    constraint: &'a ArrowConstraint<String>,
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

    let mut new_vertices = 0;
    let mut new_edges = Vec::new();

    for constraint in constraints {
        match constraint {
            Constraint::RuleObject(label, object) => match object {
                RuleObject::Vertex => new_vertices += 1,
                RuleObject::Edge { constraint } => {
                    let from = bindings.get_object(&constraint.from).unwrap();
                    let from = input_vertices
                        .iter()
                        .position(|&vertex| vertex == from)
                        .unwrap();
                    let to = bindings.get_object(&constraint.to).unwrap();
                    let to = input_vertices
                        .iter()
                        .position(|&vertex| vertex == to)
                        .unwrap();
                    new_edges.push(ArrowConstraint::new(from, to, constraint.connection));
                }
            },
            Constraint::MorphismEq(_, _) => todo!(),
        }
    }

    vec![GraphActionDo::ApplyRule {
        input_vertices,
        new_vertices,
        new_edges,
        remove_vertices: vec![],
        remove_edges: vec![],
    }]
}

// pub struct RuleBuilder<'a> {
//     pub inputs: Vec<RuleObject<&'a str>>,
//     pub constraints: Vec<RuleObject<&'a str>>,
//     pub infers: Vec<RuleObject<&'a str>>,
//     pub removes: Vec<RuleObject<&'a str>>,
//     pub outputs: Vec<RuleObject<&'a str>>,
// }

// impl<'a> RuleBuilder<'a> {
//     pub fn build(self) -> Result<Rule, RuleError> {
//         Rule::new(
//             self.inputs,
//             self.constraints,
//             self.infers,
//             self.removes,
//             self.outputs,
//         )
//     }
// }

// #[derive(Debug)]
// pub enum RuleError {
//     EmptyLabel,
// }

// impl std::error::Error for RuleError {}

// impl std::fmt::Display for RuleError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             RuleError::EmptyLabel => {
//                 write!(f, "All vertices used for connections must have a label!")
//             }
//         }
//     }
// }

// pub struct Rule {
//     inputs: Vec<RuleObject<String>>,
//     constraints: Vec<RuleObject<String>>,
//     infers: Vec<RuleObject<String>>,
//     removes: Vec<RuleObject<String>>,
//     outputs: Vec<RuleObject<String>>,
//     graph: Graph,
//     graph_input: Vec<GraphObject>,
// }

// impl Rule {
//     fn new<'a>(
//         inputs: Vec<RuleObject<&'a str>>,
//         constraints: Vec<RuleObject<&'a str>>,
//         infers: Vec<RuleObject<&'a str>>,
//         removes: Vec<RuleObject<&'a str>>,
//         outputs: Vec<RuleObject<&'a str>>,
//     ) -> Result<Self, RuleError> {
//         // Create a graph
//         let mut graph = Graph::new(ForceParameters::default());

//         let mut add_object = |object: &RuleObject<&str>,
//                               color: Color<f32>,
//                               override_color: bool|
//          -> Result<GraphObject, RuleError> {
//             match object {
//                 RuleObject::Vertex { label } => Ok(GraphObject::Vertex {
//                     id: get_vertex_id(&mut graph, label, Ok(Some(color))),
//                 }),
//                 RuleObject::Edge { label, constraint } => {
//                     // Check labels
//                     if constraint.from.is_empty() || constraint.to.is_empty() {
//                         return Err(RuleError::EmptyLabel);
//                     }

//                     let vertex_color = if override_color { Err(color) } else { Ok(None) };
//                     let from = get_vertex_id(&mut graph, constraint.from, vertex_color);
//                     let to = get_vertex_id(&mut graph, constraint.to, vertex_color);

//                     Ok(GraphObject::Edge {
//                         id: graph
//                             .graph
//                             .new_edge(ForceEdge::new(
//                                 random_shift(),
//                                 random_shift(),
//                                 ARROW_BODIES,
//                                 ARROW_MASS,
//                                 Arrow::new(label, from, to, constraint.connection, color),
//                             ))
//                             .unwrap(),
//                     })
//                 }
//             }
//         };

//         // Input
//         let mut graph_input = Vec::with_capacity(inputs.len());
//         for input in &inputs {
//             graph_input.push(add_object(input, RULE_INPUT_COLOR, false)?);
//         }

//         // Constraints
//         for constraint in &constraints {
//             add_object(constraint, RULE_INFER_CONTEXT_COLOR, true)?;
//         }

//         // Infer
//         for infer in &infers {
//             add_object(infer, RULE_INFER_COLOR, true)?;
//         }

//         // Output
//         for output in &outputs {
//             add_object(output, RULE_OUTPUT_COLOR, true)?;
//         }

//         // Removes
//         // TODO: Check validity

//         fn convert(objects: Vec<RuleObject<&str>>) -> Vec<RuleObject<String>> {
//             objects
//                 .into_iter()
//                 .map(|object| match object {
//                     RuleObject::Vertex { label } => RuleObject::Vertex {
//                         label: label.to_owned(),
//                     },
//                     RuleObject::Edge { label, constraint } => RuleObject::Edge {
//                         label: label.to_owned(),
//                         constraint: ArrowConstraint::new(
//                             constraint.from.to_owned(),
//                             constraint.to.to_owned(),
//                             constraint.connection,
//                         ),
//                     },
//                 })
//                 .collect()
//         }

//         Ok(Self {
//             inputs: convert(inputs),
//             constraints: convert(constraints),
//             infers: convert(infers),
//             outputs: convert(outputs),
//             removes: convert(removes),
//             graph,
//             graph_input,
//         })
//     }

//     pub fn inputs(&self) -> &Vec<RuleObject<String>> {
//         &self.inputs
//     }

//     pub fn constraints(&self) -> &Vec<RuleObject<String>> {
//         &self.constraints
//     }

//     pub fn get_input(&self) -> &Vec<GraphObject> {
//         &self.graph_input
//     }

//     fn get_vertex_by_label(graph: &Graph, label: &str) -> Option<VertexId> {
//         graph
//             .graph
//             .vertices
//             .iter()
//             .find(|(_, vertex)| vertex.vertex.label.eq(label))
//             .map(|(&id, _)| id)
//     }

//     pub fn graph(&self) -> &Graph {
//         &self.graph
//     }

//     pub fn graph_mut(&mut self) -> &mut Graph {
//         &mut self.graph
//     }

//     pub fn update_graph(&mut self, delta_time: f32) {
//         self.graph.update(delta_time);
//     }

//     /// Checks that input meets the rule's constraints.
//     fn check_input(&self, graph: &Graph, selection: &Vec<GraphObject>) -> bool {
//         // Check length
//         if selection.len() != self.inputs.len() {
//             return false;
//         }

//         // Check contents
//         let mut vertices = HashMap::new();
//         for object in self.inputs.iter().zip(selection.iter()) {
//             let fit = match object {
//                 (RuleObject::Vertex { label }, &GraphObject::Vertex { id }) => vertices
//                     .insert(label, id)
//                     .map(|old_id| old_id == id)
//                     .unwrap_or(true),
//                 (RuleObject::Edge { constraint, .. }, GraphObject::Edge { id }) => graph
//                     .graph
//                     .edges
//                     .get(id)
//                     .map(|edge| {
//                         edge.edge.check_constraint(&ArrowConstraint {
//                             from: *vertices.entry(&constraint.from).or_insert(edge.edge.from),
//                             to: *vertices.entry(&constraint.to).or_insert(edge.edge.to),
//                             connection: constraint.connection,
//                         })
//                     })
//                     .unwrap_or_default(),
//                 _ => false,
//             };
//             if !fit {
//                 return false;
//             }
//         }

//         true
//     }

//     /// Applies the rule
//     fn action(&self, graph: &Graph, selection: &Vec<GraphObject>) -> Result<GraphActionDo, ()> {
//         let process = RuleProcess::input(graph, self.inputs.iter(), selection.iter());
//         let constraints = process.constraint(graph, &self.constraints)?;
//         process
//             .infer(graph, constraints, &self.infers)
//             .remove(self.removes.iter())
//             .output(self.outputs.iter())
//             .action(graph)
//     }
// }

// /// Color:
// ///  - Ok(None)        -> Do nothing or use default color (inferred from context)
// ///  - Ok(Some(color)) -> Override existing color
// ///  - Err(color)      -> Create new with the given color
// fn get_vertex_id(
//     graph: &mut Graph,
//     label: &str,
//     color: Result<Option<Color<f32>>, Color<f32>>,
// ) -> VertexId {
//     match Rule::get_vertex_by_label(graph, label) {
//         Some(id) => {
//             if let Ok(Some(color)) = color {
//                 graph.graph.vertices.get_mut(&id).unwrap().vertex.color = color;
//             }
//             id
//         }
//         None => graph.graph.new_vertex(ForceVertex {
//             is_anchor: false,
//             body: ForceBody::new(random_shift(), POINT_MASS),
//             vertex: Point {
//                 label: label.to_owned(),
//                 radius: POINT_RADIUS,
//                 color: match color {
//                     Ok(color) => color.unwrap_or(RULE_INFER_CONTEXT_COLOR),
//                     Err(color) => color,
//                 },
//             },
//         }),
//     }
// }
