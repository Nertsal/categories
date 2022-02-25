use super::*;

pub fn constraint_morphism<'a, O, M, L: Label>(
    label: &'a L,
    connection: &'a MorphismConnection<L>,
    tags: &'a [MorphismTag<L, L>],
    bindings: &'a Bindings<L>,
    category: &'a Category<O, M>,
) -> Box<dyn Iterator<Item = Bindings<L>> + 'a> {
    match bindings.get_morphism(label) {
        Some(morphism) => {
            let morphism = category
                .morphisms
                .get(&morphism)
                .expect("Invalid bindings: unknown morphism id"); // TODO: return an error
            morphism_matches(connection, tags, morphism, bindings)
                .map_or(Box::new(vec![].into_iter()), |binds| {
                    Box::new(std::iter::once(binds))
                })
        }
        None => Box::new(category.morphisms.iter().filter_map(|(&id, morphism)| {
            morphism_matches(connection, tags, morphism, bindings).map(|mut binds| {
                binds.bind_morphism(label.clone(), id);
                binds
            })
        })),
    }
}

fn morphism_matches<M, L: Label>(
    connection: &MorphismConnection<L>,
    tags: &[MorphismTag<L, L>],
    morphism: &Morphism<M>,
    bindings: &Bindings<L>,
) -> Option<Bindings<L>> {
    // Check connection
    let connection_check = match (connection, &morphism.connection) {
        (
            MorphismConnection::Regular {
                from: constraint_from,
                to: constraint_to,
            },
            &MorphismConnection::Regular { from, to },
        ) => constraint_ordered(
            vec![constraint_from, constraint_to]
                .into_iter()
                .map(|label| (label.clone(), bindings.get_object(label))),
            vec![from, to],
        )
        .map(|binds| Bindings::from_objects(binds)),
        (MorphismConnection::Regular { .. }, _) => return None,
        (
            MorphismConnection::Isomorphism(constraint_f, constraint_g),
            &MorphismConnection::Isomorphism(f, g),
        ) => constraint_unordered(
            vec![constraint_f, constraint_g]
                .into_iter()
                .map(|label| (label.clone(), bindings.get_object(label))),
            vec![f, g],
        )
        .map(|binds| Bindings::from_objects(binds)),
        (MorphismConnection::Isomorphism(_, _), _) => return None,
    };

    let mut new_bindings = match connection_check {
        Some(binds) => binds,
        None => return None,
    };

    // Check tags
    for tag_check in tags
        .iter()
        .map(|constraint| tags_matches(constraint, &morphism.tags, bindings))
    {
        let binds = match tag_check {
            Some(binds) => binds,
            None => return None,
        };
        new_bindings.extend(binds);
    }

    Some(new_bindings)
}

fn tags_matches<L: Label>(
    constraint: &MorphismTag<L, L>,
    tags: &[MorphismTag],
    bindings: &Bindings<L>,
) -> Option<Bindings<L>> {
    match constraint {
        MorphismTag::Unique => tags.iter().find_map(|tag| match tag {
            MorphismTag::Unique => Some(Bindings::new()),
            _ => None,
        }),
        MorphismTag::Identity(constraint) => tags.iter().find_map(|tag| {
            if let &MorphismTag::Identity(object) = tag {
                bindings.get_object(constraint).map_or_else(
                    || Some(Bindings::single_object(constraint.clone(), object)),
                    |id| {
                        if id == object {
                            Some(Bindings::new())
                        } else {
                            None
                        }
                    },
                )
            } else {
                None
            }
        }),
        MorphismTag::Composition {
            first: constraint_first,
            second: constraint_second,
        } => tags.iter().find_map(|tag| {
            if let &MorphismTag::Composition { first, second } = tag {
                constraint_ordered(
                    vec![constraint_first, constraint_second]
                        .into_iter()
                        .map(|label| (label.clone(), bindings.get_morphism(label))),
                    vec![first, second],
                )
                .map(|binds| Bindings::from_morphisms(binds))
            } else {
                None
            }
        }),
        MorphismTag::Isomorphism(constraint_f, constraint_g) => tags.iter().find_map(|tag| {
            if let &MorphismTag::Isomorphism(morphism_f, morphism_g) = tag {
                constraint_ordered(
                    vec![constraint_f, constraint_g]
                        .into_iter()
                        .map(|label| (label.clone(), bindings.get_morphism(label))),
                    vec![morphism_f, morphism_g],
                )
                .map(|binds| Bindings::from_morphisms(binds))
            } else {
                None
            }
        }),
    }
}
