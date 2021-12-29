use super::*;

pub fn constraint_morphism<'a>(
    label: &'a Label,
    constraint: &'a ArrowConstraint,
    bindings: &'a Bindings,
    graph: &'a Graph,
) -> impl Iterator<Item = Bindings> + 'a {
    assert!(
        bindings.get_morphism(label).is_none(),
        "Morphisms must have unique names!"
    );

    let from = bindings.get_object(&constraint.from);
    let to = bindings.get_object(&constraint.to);

    graph.graph.edges.iter().filter_map(move |(&id, edge)| {
        let mut binds = Bindings::new();
        if morphism_match(from, to, &constraint.tag, edge, bindings, &mut binds) {
            binds.bind_morphism(label.to_owned(), id);
            if from.is_none() {
                binds.bind_object(constraint.from.to_owned(), edge.edge.from);
            }
            if to.is_none() {
                binds.bind_object(constraint.to.to_owned(), edge.edge.to);
            }
            Some(binds)
        } else {
            None
        }
    })
}

fn morphism_match(
    from: Option<VertexId>,
    to: Option<VertexId>,
    constraint_tag: &Option<MorphismTag>,
    edge: &Edge,
    bindings: &Bindings,
    binds: &mut Bindings,
) -> bool {
    check(edge.edge.from, from)
        && check(edge.edge.to, to)
        && match (constraint_tag, &edge.edge.tag) {
            (None, Some(MorphismTag::Isomorphism(_, _))) => false,
            (None, _) => true,
            (_, None) => false,
            (Some(MorphismTag::Unique), Some(MorphismTag::Unique)) => true,
            (Some(MorphismTag::Identity(constraint)), &Some(MorphismTag::Identity(object))) => {
                match (constraint, object) {
                    (Some(constraint), Some(object)) => match bindings.get_object(constraint) {
                        Some(constraint) => constraint == object,
                        None => {
                            binds.bind_object(constraint.clone(), object);
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
                                binds.bind_morphism(constraint_first.to_owned(), first);
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
                                binds.bind_morphism(constraint_second.to_owned(), second);
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
                        binds.bind_morphism(label, id);
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
        }
}

fn check<T: Eq>(value: T, constraint: Option<T>) -> bool {
    match constraint {
        None => true,
        Some(constraint) => value == constraint,
    }
}
