use super::*;

mod apply;
mod find;
mod init;

pub use find::*;

impl GameState {
    /// Attempts to apply a rule.
    pub fn apply_rule(&mut self, graph: FocusedCategory, selection: RuleSelection) {
        let (category, equalities) = match graph {
            FocusedCategory::Rule { .. } => return,
            FocusedCategory::Fact => (
                &mut self.fact_category.inner,
                &mut self.fact_category.equalities,
            ),
            FocusedCategory::Goal => (
                &mut self.goal_category.inner,
                &mut self.goal_category.equalities,
            ),
        };

        let rule = &self.rules[selection.rule()];
        let statement = match selection.inverse() {
            false => rule.statement(),
            true => rule.inverse_statement(),
        };

        let (actions, applied) =
            apply::rule_apply(statement, category, equalities, selection.selection());
        self.action_history.extend(actions);

        if applied && selection.inverse() {
            // TODO: smarter removal
            for morphism in selection
                .selection()
                .iter()
                .filter_map(|object| match object {
                    CategoryThing::Object { .. } => None,
                    CategoryThing::Morphism { id } => Some(id),
                })
            {
                category.morphisms.remove(morphism);
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
    graph_input: Vec<CategoryThing>,
    graph: RenderableCategory,

    inverse_statement: RuleStatement,
    inverse_graph_input: Vec<CategoryThing>,
}

impl Rule {
    pub fn graph(&self) -> &RenderableCategory {
        &self.graph
    }

    pub fn get_category_mut(&mut self) -> &mut RenderableCategory {
        &mut self.graph
    }

    pub fn statement(&self) -> &RuleStatement {
        &self.statement
    }

    pub fn inverse_statement(&self) -> &RuleStatement {
        &self.inverse_statement
    }

    pub fn graph_input(&self) -> &Vec<CategoryThing> {
        &self.graph_input
    }

    pub fn inverse_graph_input(&self) -> &Vec<CategoryThing> {
        &self.inverse_graph_input
    }
}
