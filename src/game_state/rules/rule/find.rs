use super::*;

/// Returns `None` if there are no constraints.
pub fn find_candidates<'a>(
    constraints: &'a [Constraint],
    bindings: &'a Bindings,
    category: &'a Category,
    equalities: &'a Equalities,
) -> Option<impl Iterator<Item = Bindings> + 'a> {
    let constraint = match constraints.first() {
        Some(constraint) => constraint,
        None => return None,
    };
    let constraints = &constraints[1..];

    let binds: Vec<_> = match constraint {
        Constraint::RuleObject(label, object) => match object {
            RuleObject::Object { tag } => constraint_object(label, tag, bindings, category),
            RuleObject::Morphism { constraint } => {
                constraint_morphism(label, constraint, bindings, category)
            }
        },
        Constraint::MorphismEq(morphism_f, morphism_g) => {
            constraint_equality(morphism_f, morphism_g, bindings, category, equalities)
        }
    };

    Some(binds.into_iter().flat_map(|binds| {
        let mut old_binds = binds.clone();
        old_binds.extend(bindings.clone());
        let binds = match find_candidates(constraints, &old_binds, category, equalities) {
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
