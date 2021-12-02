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

pub struct RuleBuilder<'a> {
    pub inputs: Vec<RuleObject<&'a str>>,
    pub constraints: Vec<RuleObject<&'a str>>,
    pub infers: Vec<RuleObject<&'a str>>,
    pub removes: Vec<RuleObject<&'a str>>,
    pub outputs: Vec<RuleObject<&'a str>>,
}

impl<'a> RuleBuilder<'a> {
    pub fn build(self) -> Result<Rule, RuleError> {
        Rule::new(
            self.inputs,
            self.constraints,
            self.infers,
            self.removes,
            self.outputs,
        )
    }
}

#[derive(Debug)]
pub enum RuleError {
    EmptyLabel,
}

impl std::error::Error for RuleError {}

impl std::fmt::Display for RuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleError::EmptyLabel => {
                write!(f, "All vertices used for connections must have a label!")
            }
        }
    }
}

pub struct Rule {
    inputs: Vec<RuleObject<String>>,
    constraints: Vec<RuleObject<String>>,
    infers: Vec<RuleObject<String>>,
    removes: Vec<RuleObject<String>>,
    outputs: Vec<RuleObject<String>>,
    graph: Graph,
    graph_input: Vec<GraphObject>,
}

impl Rule {
    fn new<'a>(
        inputs: Vec<RuleObject<&'a str>>,
        constraints: Vec<RuleObject<&'a str>>,
        infers: Vec<RuleObject<&'a str>>,
        removes: Vec<RuleObject<&'a str>>,
        outputs: Vec<RuleObject<&'a str>>,
    ) -> Result<Self, RuleError> {
        // Create a graph
        let mut graph = Graph::new(ForceParameters::default());

        let mut add_object = |object: &RuleObject<&str>,
                              color: Color<f32>,
                              override_color: bool|
         -> Result<GraphObject, RuleError> {
            match object {
                RuleObject::Vertex { label } => Ok(GraphObject::Vertex {
                    id: get_vertex_id(&mut graph, label, Ok(Some(color))),
                }),
                RuleObject::Edge { label, constraint } => {
                    // Check labels
                    if constraint.from.is_empty() || constraint.to.is_empty() {
                        return Err(RuleError::EmptyLabel);
                    }

                    let vertex_color = if override_color { Err(color) } else { Ok(None) };
                    let from = get_vertex_id(&mut graph, constraint.from, vertex_color);
                    let to = get_vertex_id(&mut graph, constraint.to, vertex_color);

                    Ok(GraphObject::Edge {
                        id: graph
                            .graph
                            .new_edge(ForceEdge::new(
                                random_shift(),
                                random_shift(),
                                ARROW_BODIES,
                                ARROW_MASS,
                                Arrow::new(label, from, to, constraint.connection, color),
                            ))
                            .unwrap(),
                    })
                }
            }
        };

        // Input
        let mut graph_input = Vec::with_capacity(inputs.len());
        for input in &inputs {
            graph_input.push(add_object(input, RULE_INPUT_COLOR, false)?);
        }

        // Constraints
        for constraint in &constraints {
            add_object(constraint, RULE_INFER_CONTEXT_COLOR, true)?;
        }

        // Infer
        for infer in &infers {
            add_object(infer, RULE_INFER_COLOR, true)?;
        }

        // Output
        for output in &outputs {
            add_object(output, RULE_OUTPUT_COLOR, true)?;
        }

        // Removes
        // TODO: Check validity

        fn convert(objects: Vec<RuleObject<&str>>) -> Vec<RuleObject<String>> {
            objects
                .into_iter()
                .map(|object| match object {
                    RuleObject::Vertex { label } => RuleObject::Vertex {
                        label: label.to_owned(),
                    },
                    RuleObject::Edge { label, constraint } => RuleObject::Edge {
                        label: label.to_owned(),
                        constraint: ArrowConstraint::new(
                            constraint.from.to_owned(),
                            constraint.to.to_owned(),
                            constraint.connection,
                        ),
                    },
                })
                .collect()
        }

        Ok(Self {
            inputs: convert(inputs),
            constraints: convert(constraints),
            infers: convert(infers),
            outputs: convert(outputs),
            removes: convert(removes),
            graph,
            graph_input,
        })
    }

    pub fn inputs(&self) -> &Vec<RuleObject<String>> {
        &self.inputs
    }

    pub fn constraints(&self) -> &Vec<RuleObject<String>> {
        &self.constraints
    }

    pub fn get_input(&self) -> &Vec<GraphObject> {
        &self.graph_input
    }

    fn get_vertex_by_label(graph: &Graph, label: &str) -> Option<VertexId> {
        graph
            .graph
            .vertices
            .iter()
            .find(|(_, vertex)| vertex.vertex.label.eq(label))
            .map(|(&id, _)| id)
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

    /// Checks that input meets the rule's constraints.
    fn check_input(&self, graph: &Graph, selection: &Vec<GraphObject>) -> bool {
        // Check length
        if selection.len() != self.inputs.len() {
            return false;
        }

        // Check contents
        let mut vertices = HashMap::new();
        for object in self.inputs.iter().zip(selection.iter()) {
            let fit = match object {
                (RuleObject::Vertex { label }, &GraphObject::Vertex { id }) => vertices
                    .insert(label, id)
                    .map(|old_id| old_id == id)
                    .unwrap_or(true),
                (RuleObject::Edge { constraint, .. }, GraphObject::Edge { id }) => graph
                    .graph
                    .edges
                    .get(id)
                    .map(|edge| {
                        edge.edge.check_constraint(&ArrowConstraint {
                            from: *vertices.entry(&constraint.from).or_insert(edge.edge.from),
                            to: *vertices.entry(&constraint.to).or_insert(edge.edge.to),
                            connection: constraint.connection,
                        })
                    })
                    .unwrap_or_default(),
                _ => false,
            };
            if !fit {
                return false;
            }
        }

        true
    }

    /// Applies the rule
    fn action(&self, graph: &Graph, selection: &Vec<GraphObject>) -> Result<GraphActionDo, ()> {
        let process = RuleProcess::input(graph, self.inputs.iter(), selection.iter());
        let constraints = process.constraint(graph, &self.constraints)?;
        process
            .infer(graph, constraints, &self.infers)
            .remove(self.removes.iter())
            .output(self.outputs.iter())
            .action(graph)
    }
}

/// Color:
///  - Ok(None)        -> Do nothing or use default color (inferred from context)
///  - Ok(Some(color)) -> Override existing color
///  - Err(color)      -> Create new with the given color
fn get_vertex_id(
    graph: &mut Graph,
    label: &str,
    color: Result<Option<Color<f32>>, Color<f32>>,
) -> VertexId {
    match Rule::get_vertex_by_label(graph, label) {
        Some(id) => {
            if let Ok(Some(color)) = color {
                graph.graph.vertices.get_mut(&id).unwrap().vertex.color = color;
            }
            id
        }
        None => graph.graph.new_vertex(ForceVertex {
            is_anchor: false,
            body: ForceBody::new(random_shift(), POINT_MASS),
            vertex: Point {
                label: label.to_owned(),
                radius: POINT_RADIUS,
                color: match color {
                    Ok(color) => color.unwrap_or(RULE_INFER_CONTEXT_COLOR),
                    Err(color) => color,
                },
            },
        }),
    }
}
