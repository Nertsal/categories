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

pub struct Rule {
    inputs: Vec<RuleObject<String>>,
    infers: Vec<RuleObject<String>>,
    outputs: Vec<RuleObject<String>>,
    removes: Vec<RuleObject<String>>,
    graph: Graph,
    graph_input: Vec<GraphObject>,
}

impl Rule {
    pub fn new<'a>(
        inputs: Vec<RuleObject<&'a str>>,
        infers: Vec<RuleObject<&'a str>>,
        outputs: Vec<RuleObject<&'a str>>,
        removes: Vec<RuleObject<&'a str>>,
    ) -> Self {
        // Create a graph
        let mut graph = Graph::new(ForceParameters::default());

        let mut add_object =
            |object: &RuleObject<&str>, color: Color<f32>, override_color: bool| match object {
                RuleObject::Vertex { label } => GraphObject::Vertex {
                    id: get_vertex_id(&mut graph, label, Ok(Some(color))),
                },
                RuleObject::Edge { label, constraint } => {
                    let vertex_color = if override_color { Err(color) } else { Ok(None) };
                    let from = get_vertex_id(&mut graph, &constraint.from, vertex_color);
                    let to = get_vertex_id(&mut graph, &constraint.to, vertex_color);
                    GraphObject::Edge {
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
                    }
                }
            };

        // Input
        let graph_input = inputs
            .iter()
            .map(|input| add_object(input, RULE_INPUT_COLOR, false))
            .collect();

        // Infer
        for infer in &infers {
            add_object(infer, RULE_INFER_COLOR, true);
        }

        // Output
        for output in &outputs {
            add_object(output, RULE_OUTPUT_COLOR, true);
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

        Self {
            inputs: convert(inputs),
            infers: convert(infers),
            outputs: convert(outputs),
            removes: convert(removes),
            graph,
            graph_input,
        }
    }

    pub fn inputs(&self) -> &Vec<RuleObject<String>> {
        &self.inputs
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
        RuleProcess::input(graph, self.inputs.iter(), selection.iter())
            .infer(graph, self.infers.iter())
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
