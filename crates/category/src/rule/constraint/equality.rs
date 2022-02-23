use std::collections::VecDeque;

use super::*;

pub fn constraint_equality<'a, O, M, L: Label>(
    equality: &'a Equality<L>,
    bindings: &'a Bindings<L>,
    category: &'a Category<O, M>,
) -> Box<dyn Iterator<Item = Bindings<L>> + 'a> {
    let [left_constraints, right_constraints] =
        [equality.left(), equality.right()].map(|eq_side| {
            eq_side
                .iter()
                .map(|label| (label.clone(), bindings.get_morphism(label)))
        });

    let right_constraints = right_constraints.collect::<Vec<_>>();

    Box::new(
        find_possibilities(left_constraints, category).flat_map(move |left| {
            find_possibilities(right_constraints.clone().into_iter(), category)
                .filter_map(move |right| check_equality(left.clone(), right, category))
        }),
    )
}

fn find_possibilities<'a, O, M, L: 'a + Clone>(
    mut constraints: impl Iterator<Item = (L, Option<MorphismId>)> + 'a,
    category: &'a Category<O, M>,
) -> Box<dyn Iterator<Item = VecDeque<(L, MorphismId)>> + 'a> {
    match constraints.next() {
        None => Box::new(std::iter::once(VecDeque::new())),
        Some((label, Some(id))) => Box::new(find_possibilities(constraints, category).map(
            move |mut tail| {
                tail.push_front((label.clone(), id));
                tail
            },
        )),
        Some((label, None)) => {
            let head = category
                .morphisms
                .iter()
                .map(|(&id, _)| (label.clone(), id))
                .collect::<Vec<_>>();
            Box::new(
                find_possibilities(constraints, category).flat_map(move |tail| {
                    head.clone().into_iter().map(move |head| {
                        let mut tail = tail.clone();
                        tail.push_front(head);
                        tail
                    })
                }),
            )
        }
    }
}

fn check_equality<O, M, L: Label>(
    left: impl IntoIterator<Item = (L, MorphismId)>,
    right: impl IntoIterator<Item = (L, MorphismId)>,
    category: &Category<O, M>,
) -> Option<Bindings<L>> {
    let mut bindings = Bindings::new();
    let left = left.into_iter().fold(Vec::new(), |mut acc, (label, id)| {
        bindings.bind_morphism(label, id);
        acc.push(id);
        acc
    });
    let right = right.into_iter().fold(Vec::new(), |mut acc, (label, id)| {
        bindings.bind_morphism(label, id);
        acc.push(id);
        acc
    });

    if !check_composability(left.iter().copied(), category)
        || !check_composability(right.iter().copied(), category)
    {
        return None;
    }
    // At this point morphisms are guaranteed to exist and be composable

    let remove_ids = |morphisms: Vec<MorphismId>| {
        let len = morphisms.len();
        let mut morphisms = morphisms.into_iter();
        let mut result = (0..len - 1)
            .map(|_| morphisms.next().unwrap())
            .filter(|morphism| check_identity(morphism, category).is_none())
            .collect::<Vec<_>>();
        let morphism = morphisms.next().unwrap();
        if result.is_empty() || check_identity(&morphism, category).is_none() {
            result.push(morphism);
        }
        result
    };

    let decompose = |composition: Vec<MorphismId>| {
        composition
            .into_iter()
            .flat_map(|id| decompose_morphism(id, category.morphisms.get(&id).unwrap(), category))
            .collect()
    };

    let left = remove_ids(decompose(left));
    let right = remove_ids(decompose(right));
    // At this point there are no identity morphisms

    if left == right
        || category
            .equalities
            .contains_equality(&Equality::new(left, right).unwrap())
    {
        Some(bindings)
    } else {
        None
    }
}

fn check_identity<O, M>(morphism: &MorphismId, category: &Category<O, M>) -> Option<ObjectId> {
    category
        .morphisms
        .get(morphism)
        .unwrap()
        .tags
        .iter()
        .find_map(|tag| match tag {
            &MorphismTag::Identity(object) => Some(object),
            _ => None,
        })
}

fn check_composability<O, M>(
    morphisms: impl IntoIterator<Item = MorphismId>,
    category: &Category<O, M>,
) -> bool {
    let mut morphisms = morphisms.into_iter();

    let mut from = match morphisms.next() {
        None => return true,
        Some(id) => match category.morphisms.get(&id) {
            None => return false,
            Some(morphism) => *morphism.connection.end_points()[1],
        },
    };

    for morphism in morphisms {
        match category.morphisms.get(&morphism) {
            None => return false,
            Some(morphism) => {
                if from != *morphism.connection.end_points()[0] {
                    return false;
                } else {
                    from = *morphism.connection.end_points()[1];
                }
            }
        }
    }

    true
}
