use super::*;

mod apply;
mod init;

impl GameState {
    /// Attempts to apply a rule.
    /// Returns whether the rule was applied successfully.
    pub fn apply_rule(&mut self, graph: FocusedGraph, selection: RuleSelection) {
        let graph = match graph {
            FocusedGraph::Rule { .. } => return,
            FocusedGraph::Main => &mut self.main_graph.graph,
            FocusedGraph::Goal => &mut self.goal_graph.graph,
        };

        let rule = &self.rules[selection.rule()];
        let actions = rule.apply(graph, selection.selection());
        self.action_history.extend(actions);
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

    pub fn build(self, geng: &Geng, assets: &Rc<Assets>) -> Rule {
        Rule::new(geng, assets, self.statement)
    }
}

pub struct Rule {
    statement: RuleStatement,
    graph_input: Vec<GraphObject>,
    graph: RenderableGraph,

    inverse_statement: RuleStatement,
    inverse_graph_input: Vec<GraphObject>,
}

impl Rule {
    pub fn graph(&self) -> &RenderableGraph {
        &self.graph
    }

    pub fn graph_mut(&mut self) -> &mut RenderableGraph {
        &mut self.graph
    }

    pub fn update_graph(&mut self, delta_time: f32) {
        self.graph.graph.update(delta_time);
    }

    pub fn statement(&self) -> &RuleStatement {
        &self.statement
    }

    pub fn inverse_statement(&self) -> &RuleStatement {
        &self.inverse_statement
    }

    pub fn graph_input(&self) -> &Vec<GraphObject> {
        &self.graph_input
    }

    pub fn inverse_graph_input(&self) -> &Vec<GraphObject> {
        &self.inverse_graph_input
    }
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
