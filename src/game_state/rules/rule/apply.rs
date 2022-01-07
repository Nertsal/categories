use super::*;

impl Rule {
    /// Attempts to apply the rule and returns the action history (undo actions) and whether the rule was applied successfully.
    pub(super) fn apply(
        statement: &[RuleConstruction],
        graph: &mut Graph,
        graph_equalities: &mut GraphEqualities,
        selection: &Vec<GraphObject>,
    ) -> (Vec<GraphAction>, bool) {
        let bindings = match statement.first() {
            Some(RuleConstruction::Forall(constraints))
            | Some(RuleConstruction::Exists(constraints)) => {
                match selection_constraints(selection, constraints, graph, graph_equalities) {
                    Ok(bindings) => bindings,
                    Err(_) => return (Vec::new(), false),
                }
            }
            _ => Bindings::new(),
        };

        apply_impl(statement, bindings, graph, graph_equalities)
    }
}

fn apply_impl(
    statement: &[RuleConstruction],
    bindings: Bindings,
    graph: &mut Graph,
    graph_equalities: &mut GraphEqualities,
) -> (Vec<GraphAction>, bool) {
    let construction = match statement.first() {
        Some(construction) => construction,
        None => return (Vec::new(), false),
    };

    let statement = &statement[1..];
    match construction {
        RuleConstruction::Forall(constraints) => {
            find_candidates(constraints, &bindings, graph, graph_equalities)
                .map(|candidates| candidates.collect::<Vec<_>>())
                .unwrap_or_else(|| vec![Bindings::new()])
                .into_iter()
                .map(|mut binds| {
                    binds.extend(bindings.clone());
                    apply_impl(statement, binds, graph, graph_equalities)
                })
                .fold(
                    (Vec::new(), false),
                    |(mut acc_actions, acc_apply), (action, apply)| {
                        acc_actions.extend(action);
                        (acc_actions, acc_apply || apply)
                    },
                )
        }
        RuleConstruction::Exists(constraints) => {
            match find_candidates(constraints, &bindings, graph, graph_equalities)
                .map(|mut candidates| candidates.next())
                .unwrap_or(Some(Bindings::new()))
            {
                Some(mut binds) => {
                    binds.extend(bindings);
                    (
                        apply_impl(statement, binds, graph, graph_equalities).0,
                        true,
                    )
                }
                None => {
                    let (mut actions, new_binds) =
                        apply_constraints(graph, graph_equalities, constraints, &bindings);
                    actions.extend(apply_impl(statement, new_binds, graph, graph_equalities).0);
                    (actions, true)
                }
            }
        }
    }
}
