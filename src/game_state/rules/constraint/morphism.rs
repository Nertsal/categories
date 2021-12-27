use super::*;

pub fn constraint_morphism<'a>(
    label: &'a RuleLabel,
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

    fn check<T: Eq>(value: T, constraint: Option<T>) -> bool {
        match constraint {
            None => true,
            Some(constraint) => value == constraint,
        }
    }

    graph.graph.edges.iter().filter_map(move |(&id, edge)| {
        let mut binds = Bindings::new();
        if check(edge.edge.from, from)
            && check(edge.edge.to, to)
            && constraint.tags.iter().all(|constraint| {
                edge.edge.tags.iter().any(|tag| match (constraint, tag) {
                    (MorphismTag::Unique, MorphismTag::Unique) => true,
                    (
                        MorphismTag::Identity(Some(constraint)),
                        &MorphismTag::Identity(Some(object)),
                    ) => match bindings.get_object(constraint) {
                        Some(constraint) => constraint == object,
                        None => {
                            binds.bind_object(constraint.clone(), object);
                            true
                        }
                    },
                    (
                        MorphismTag::Composition {
                            first: Some(constraint_first),
                            second: Some(constraint_second),
                        },
                        &MorphismTag::Composition {
                            first: Some(first),
                            second: Some(second),
                        },
                    ) => {
                        let match_first = match bindings.get_morphism(constraint_first) {
                            Some(constraint) => constraint == first,
                            None => {
                                binds.bind_morphism(constraint_first.to_owned(), first);
                                true
                            }
                        };

                        let match_second = match bindings.get_morphism(constraint_second) {
                            Some(constraint) => constraint == second,
                            None => {
                                binds.bind_morphism(constraint_second.to_owned(), second);
                                true
                            }
                        };

                        match_first && match_second
                    }
                    (
                        MorphismTag::Isomorphism(Some(constraint0), Some(constraint1)),
                        &MorphismTag::Isomorphism(Some(morphism0), Some(morphism1)),
                    ) => {
                        match (
                            bindings.get_morphism(constraint0),
                            bindings.get_morphism(constraint1),
                        ) {
                            (Some(constraint0), Some(constraint1)) => {
                                constraint0 == morphism0 && constraint1 == morphism1
                                    || constraint0 == morphism1 && constraint1 == morphism0
                            }
                            (Some(constraint0), None) => {
                                if constraint0 == morphism0 {
                                    binds.bind_morphism(constraint1.to_owned(), morphism1);
                                    true
                                } else if constraint0 == morphism1 {
                                    binds.bind_morphism(constraint1.to_owned(), morphism0);
                                    true
                                } else {
                                    false
                                }
                            }
                            (None, Some(constraint1)) => {
                                if constraint1 == morphism0 {
                                    binds.bind_morphism(constraint0.to_owned(), morphism1);
                                    true
                                } else if constraint1 == morphism1 {
                                    binds.bind_morphism(constraint0.to_owned(), morphism0);
                                    true
                                } else {
                                    false
                                }
                            }
                            (None, None) => {
                                binds.bind_morphism(constraint0.to_owned(), morphism0);
                                binds.bind_morphism(constraint1.to_owned(), morphism1);
                                true
                            }
                        }
                    }
                    _ => false,
                })
            })
        {
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
