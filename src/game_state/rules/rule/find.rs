use super::*;

/// Returns `None` if there are no constraints.
pub fn find_candidates<'a>(
    constraints: &'a [Constraint],
    bindings: &'a Bindings,
    graph: &'a Graph,
    equalities: &'a GraphEqualities,
) -> Option<impl Iterator<Item = Bindings> + 'a> {
    let constraint = match constraints.first() {
        Some(constraint) => constraint,
        None => return None,
    };
    let constraints = &constraints[1..];

    let binds: Vec<_> = match constraint {
        Constraint::RuleObject(label, object) => match object {
            RuleObject::Vertex { tag } => constraint_object(label, tag, bindings, graph),
            RuleObject::Edge { constraint } => {
                constraint_morphism(label, constraint, bindings, graph)
            }
        },
        Constraint::MorphismEq(morphism_f, morphism_g) => {
            constraint_equality(morphism_f, morphism_g, bindings, graph, equalities)
        }
    };

    Some(binds.into_iter().flat_map(|binds| {
        let mut old_binds = binds.clone();
        old_binds.extend(bindings.clone());
        let binds = match find_candidates(constraints, &old_binds, graph, equalities) {
            Some(new_binds) => new_binds
                .map(|mut next_binds| {
                    next_binds.extend(binds.clone());
                    next_binds
                })
                .collect::<Vec<_>>(),
            None => vec![binds.clone()],
        };
        binds
    }))
}
