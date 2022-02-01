use super::*;

/// Returns `None` if there are no constraints.
pub fn find_candidates<'a, O, M, L: Label>(
    constraints: &'a [Constraint<L>],
    bindings: &'a Bindings<L>,
    category: &'a Category<O, M>,
) -> Option<Box<dyn Iterator<Item = Bindings<L>> + 'a>> {
    let constraint = match constraints.first() {
        Some(constraint) => constraint,
        None => return None,
    };
    let constraints = &constraints[1..];

    let binds: Box<dyn Iterator<Item = _>> = match constraint {
        Constraint::Object { label, tags } => Box::new(process(
            constraint_object(label, tags, bindings, category),
            constraints,
            bindings,
            category,
        )),
        Constraint::Morphism {
            label,
            connection,
            tags,
        } => Box::new(process(
            constraint_morphism(label, connection, tags, bindings, category),
            constraints,
            bindings,
            category,
        )),
        Constraint::Equality(f, g) => Box::new(process(
            constraint_equality(f, g, bindings, category),
            constraints,
            bindings,
            category,
        )),
        Constraint::Commute { f, g, h } => Box::new(process(
            constraint_commute(f, g, h, bindings, category),
            constraints,
            bindings,
            category,
        )),
    };

    Some(binds)
}

fn process<'a, O, M, L: Label>(
    new_binds: impl Iterator<Item = Bindings<L>> + 'a,
    constraints: &'a [Constraint<L>],
    bindings: &'a Bindings<L>,
    category: &'a Category<O, M>,
) -> impl Iterator<Item = Bindings<L>> + 'a {
    new_binds.flat_map(|binds| {
        let mut old_binds = binds.clone();
        old_binds.extend(bindings.clone());
        let binds = match find_candidates(constraints, &old_binds, category) {
            Some(new_binds) => new_binds
                .map(|mut next_binds| {
                    next_binds.extend(binds.clone());
                    next_binds
                })
                .collect::<Vec<_>>(),
            None => vec![binds.clone()],
        };
        binds
    })
}
