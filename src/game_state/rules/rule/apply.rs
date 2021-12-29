use super::*;

impl Rule {
    fn apply_impl(
        statement: &[RuleConstruction],
        bindings: Bindings,
        graph: &mut Graph,
    ) -> Vec<GraphAction> {
        let construction = match statement.first() {
            Some(construction) => construction,
            None => return Vec::new(),
        };

        let statement = &statement[1..];
        match construction {
            RuleConstruction::Forall(constraints) => find_candidates(constraints, &bindings, graph)
                .into_iter()
                .map(|mut binds| {
                    binds.extend(bindings.clone());
                    Self::apply_impl(statement, binds, graph)
                })
                .flatten()
                .collect(),
            RuleConstruction::Exists(constraints) => {
                match find_candidates(constraints, &bindings, graph)
                    .into_iter()
                    .next()
                {
                    Some(mut binds) => {
                        binds.extend(bindings);
                        Self::apply_impl(statement, binds, graph)
                    }
                    None => {
                        let (mut actions, new_binds) =
                            apply_constraints(graph, constraints, &bindings);
                        actions.extend(Self::apply_impl(statement, new_binds, graph));
                        actions
                    }
                }
            }
        }
    }

    /// Attempts to apply the rule and returns the action history (undo actions).
    pub(super) fn apply(
        statement: &[RuleConstruction],
        graph: &mut Graph,
        selection: &Vec<GraphObject>,
    ) -> Vec<GraphAction> {
        let bindings = match statement.first() {
            Some(RuleConstruction::Forall(constraints))
            | Some(RuleConstruction::Exists(constraints)) => {
                match selection_constraints(selection, constraints, graph) {
                    Ok(bindings) => bindings,
                    Err(_) => return Vec::new(),
                }
            }
            _ => Bindings::new(),
        };

        Self::apply_impl(statement, bindings, graph)
    }
}
