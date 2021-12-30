use super::*;

mod apply;
mod find;
mod init;

pub use find::*;

impl GameState {
    /// Attempts to apply a rule.
    pub fn apply_rule(&mut self, graph: FocusedGraph, selection: RuleSelection) {
        let graph = match graph {
            FocusedGraph::Rule { .. } => return,
            FocusedGraph::Main => &mut self.main_graph.graph,
            FocusedGraph::Goal => &mut self.goal_graph.graph,
        };

        let rule = &self.rules[selection.rule()];
        let statement = match selection.inverse() {
            false => rule.statement(),
            true => rule.inverse_statement(),
        };

        let (actions, applied) = Rule::apply(statement, graph, selection.selection());
        self.action_history.extend(actions);

        if applied && selection.inverse() {
            // TODO: smarter removal
            for edge in selection
                .selection()
                .iter()
                .filter_map(|object| match object {
                    GraphObject::Vertex { .. } => None,
                    GraphObject::Edge { id } => Some(id),
                })
            {
                graph.graph.edges.remove(edge);
            }
        }

        self.check_goal();
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
