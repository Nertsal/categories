use super::*;

pub fn constraint_morphism(
    label: &Label,
    constraint: &ArrowConstraint,
    bindings: &Bindings,
    category: &Category,
) -> Vec<Bindings> {
    match bindings.get_morphism(label) {
        Some(morphism_id) => {
            let morphism = category.morphisms.get(&morphism_id).unwrap();
            if let Some(binds) = morphism_match(constraint, morphism, bindings) {
                vec![binds]
            } else {
                vec![]
            }
        }
        None => category
            .morphisms
            .iter()
            .filter_map(move |(&id, morphism)| {
                if let Some(mut binds) = morphism_match(constraint, morphism, bindings) {
                    binds.bind_morphism(label.clone(), id);
                    Some(binds)
                } else {
                    None
                }
            })
            .collect(),
    }
}

fn morphism_match(
    constraint: &ArrowConstraint,
    morphism: &Morphism,
    bindings: &Bindings,
) -> Option<Bindings> {
    let mut new_binds = Bindings::new();
    let objects = match (morphism.connection, &constraint.connection) {
        (
            MorphismConnection::Regular { from, to },
            MorphismConnection::Regular {
                from: constraint_from,
                to: constraint_to,
            },
        ) => [(constraint_from, from), (constraint_to, to)],
        (
            MorphismConnection::Isomorphism(a, b),
            MorphismConnection::Isomorphism(constraint_a, constraint_b),
        ) => [(constraint_a, a), (constraint_b, b)],
        _ => return None,
    };

    let fit = objects.into_iter().all(|(label, id)| {
        bindings
            .get_object(label)
            .map(|constraint| id == constraint)
            .unwrap_or_else(|| {
                new_binds.bind_object(label.clone(), id);
                true
            })
    }) && match (&constraint.tag, &morphism.inner.tag) {
        (None, Some(MorphismTag::Isomorphism(_, _))) => false,
        (None, _) => true,
        (Some(_), None) => false,
        (Some(MorphismTag::Unique), _) => true,
        (Some(MorphismTag::Identity(constraint)), &Some(MorphismTag::Identity(object))) => {
            match (constraint, object) {
                (Some(constraint), Some(object)) => match bindings.get_object(constraint) {
                    Some(constraint) => constraint == object,
                    None => {
                        new_binds.bind_object(constraint.clone(), object);
                        true
                    }
                },
                _ => true,
            }
        }
        (
            Some(MorphismTag::Composition {
                first: constraint_first,
                second: constraint_second,
            }),
            &Some(MorphismTag::Composition { first, second }),
        ) => {
            let match_first = match (constraint_first, first) {
                (Some(constraint_first), Some(first)) => {
                    match bindings.get_morphism(constraint_first) {
                        Some(constraint) => constraint == first,
                        None => {
                            new_binds.bind_morphism(constraint_first.clone(), first);
                            true
                        }
                    }
                }
                _ => true,
            };

            let match_second = match (constraint_second, second) {
                (Some(constraint_second), Some(second)) => {
                    match bindings.get_morphism(constraint_second) {
                        Some(constraint) => constraint == second,
                        None => {
                            new_binds.bind_morphism(constraint_second.clone(), second);
                            true
                        }
                    }
                }
                _ => true,
            };

            match_first && match_second
        }
        (
            Some(MorphismTag::Isomorphism(constraint0, constraint1)),
            &Some(MorphismTag::Isomorphism(morphism0, morphism1)),
        ) => {
            let mut bind = |label, id| match (label, id) {
                (Some(label), Some(id)) => {
                    new_binds.bind_morphism(label, id);
                }
                _ => (),
            };

            match (
                constraint0
                    .as_ref()
                    .and_then(|constraint| bindings.get_morphism(constraint)),
                constraint1
                    .as_ref()
                    .and_then(|constraint| bindings.get_morphism(constraint)),
            ) {
                (Some(constraint0), Some(constraint1)) => {
                    check(constraint0, morphism0) && check(constraint1, morphism1)
                        || check(constraint0, morphism1) && check(constraint1, morphism0)
                }
                (Some(constraint0), None) => {
                    if check(constraint0, morphism0) {
                        bind(constraint1.clone(), morphism1);
                        true
                    } else if check(constraint0, morphism1) {
                        bind(constraint1.clone(), morphism0);
                        true
                    } else {
                        false
                    }
                }
                (None, Some(constraint1)) => {
                    if check(constraint1, morphism0) {
                        bind(constraint0.clone(), morphism1);
                        true
                    } else if check(constraint1, morphism1) {
                        bind(constraint0.clone(), morphism0);
                        true
                    } else {
                        false
                    }
                }
                (None, None) => {
                    bind(constraint0.clone(), morphism0);
                    bind(constraint1.clone(), morphism1);
                    true
                }
            }
        }
        _ => false,
    };

    if fit {
        Some(new_binds)
    } else {
        None
    }
}

fn check<T: Eq>(value: T, constraint: Option<T>) -> bool {
    match constraint {
        None => true,
        Some(constraint) => value == constraint,
    }
}
