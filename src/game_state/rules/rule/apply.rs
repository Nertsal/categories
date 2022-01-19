use super::*;

/// Attempts to apply the rule and returns the action history (undo actions) and whether the rule was applied successfully.
pub(super) fn rule_apply(
    statement: &[RuleConstruction],
    category: &mut Category,
    equalities: &mut Equalities,
    selection: &Vec<CategoryThing>,
) -> (Vec<GraphAction>, bool) {
    let bindings = match statement.first() {
        Some(RuleConstruction::Forall(constraints))
        | Some(RuleConstruction::Exists(constraints)) => {
            match selection_constraints(selection, constraints, category, equalities) {
                Ok(bindings) => bindings,
                Err(_) => return (Vec::new(), false),
            }
        }
        _ => Bindings::new(),
    };

    apply_impl(statement, bindings, category, equalities)
}

fn apply_impl(
    statement: &[RuleConstruction],
    bindings: Bindings,
    category: &mut Category,
    equalities: &mut Equalities,
) -> (Vec<GraphAction>, bool) {
    let construction = match statement.first() {
        Some(construction) => construction,
        None => return (Vec::new(), false),
    };

    let statement = &statement[1..];
    match construction {
        RuleConstruction::Forall(constraints) => {
            find_candidates(constraints, &bindings, category, equalities)
                .map(|candidates| candidates.collect::<Vec<_>>())
                .unwrap_or_else(|| vec![Bindings::new()])
                .into_iter()
                .map(|mut binds| {
                    binds.extend(bindings.clone());
                    apply_impl(statement, binds, category, equalities)
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
            let candidates = find_candidates(constraints, &bindings, category, equalities)
                .map(|candidates| candidates.collect::<Vec<_>>())
                .unwrap_or_else(|| vec![Bindings::new()]);

            if candidates.is_empty() {
                let (mut actions, new_binds) =
                    apply_constraints(category, equalities, constraints, &bindings);
                actions.extend(apply_impl(statement, new_binds, category, equalities).0);
                (actions, true)
            } else {
                candidates
                    .into_iter()
                    .map(|mut binds| {
                        binds.extend(bindings.clone());
                        (apply_impl(statement, binds, category, equalities).0, true)
                    })
                    .fold(
                        (Vec::new(), false),
                        |(mut acc_actions, acc_apply), (action, apply)| {
                            acc_actions.extend(action);
                            (acc_actions, acc_apply || apply)
                        },
                    )
            }
        }
    }
}
