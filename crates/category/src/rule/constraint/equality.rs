use std::collections::VecDeque;

use super::*;

pub fn constraint_equality<'a, O, M, E, L: Label>(
    equality: &'a Equality<L>,
    bindings: &'a Bindings<L>,
    category: &'a Category<O, M, E>,
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

fn find_possibilities<'a, O, M, E, L: 'a + Clone>(
    mut constraints: impl Iterator<Item = (L, Option<MorphismId>)> + 'a,
    category: &'a Category<O, M, E>,
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

fn check_equality<O, M, E, L: Label>(
    left: impl IntoIterator<Item = (L, MorphismId)>,
    right: impl IntoIterator<Item = (L, MorphismId)>,
    category: &Category<O, M, E>,
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

    if left
        .iter()
        .chain(right.iter())
        .any(|id| is_isomorphism(id, category))
    {
        return None;
    }

    if !check_composability(left.iter().copied(), category)
        || !check_composability(right.iter().copied(), category)
    {
        return None;
    }
    // At this point morphisms are guaranteed to exist and be composable

    let decompose = |composition: Vec<MorphismId>| {
        composition
            .into_iter()
            .flat_map(|id| decompose_morphism(id, category))
            .collect()
    };

    let left = remove_ids(decompose(left), category);
    let right = remove_ids(decompose(right), category);
    // At this point there are no identity morphisms

    if solve_equality(left, right, category) {
        Some(bindings)
    } else {
        None
    }
}

fn remove_ids<O, M, E>(
    morphisms: Vec<MorphismId>,
    category: &Category<O, M, E>,
) -> Vec<MorphismId> {
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
}

fn check_identity<O, M, E>(
    morphism: &MorphismId,
    category: &Category<O, M, E>,
) -> Option<ObjectId> {
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

fn is_isomorphism<O, M, E>(morphism: &MorphismId, category: &Category<O, M, E>) -> bool {
    match &category.morphisms.get(morphism).unwrap().connection {
        MorphismConnection::Isomorphism(_, _) => true,
        MorphismConnection::Regular { .. } => false,
    }
}

fn check_composability<O, M, E>(
    morphisms: impl IntoIterator<Item = MorphismId>,
    category: &Category<O, M, E>,
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

fn solve_equality<O, M, E>(
    mut left: Vec<MorphismId>,
    mut right: Vec<MorphismId>,
    category: &Category<O, M, E>,
) -> bool {
    if left == right {
        return true;
    }
    if left.len() < right.len() {
        std::mem::swap(&mut left, &mut right);
    }

    for equality in category.equalities.iter_equalities() {
        let mut left_eq = equality.left();
        let mut right_eq = equality.right();
        if left_eq.len() < right_eq.len() {
            std::mem::swap(&mut left_eq, &mut right_eq);
        }

        if let Some(new_left) = apply_equality(&left, left_eq, right_eq) {
            if solve_equality(remove_ids(new_left, category), right.clone(), category) {
                return true;
            }
        }
        if let Some(new_right) = apply_equality(&right, left_eq, right_eq) {
            if solve_equality(left.clone(), remove_ids(new_right, category), category) {
                return true;
            }
        }
    }

    false
}

fn apply_equality(
    data: &[MorphismId],
    left_eq: &Vec<MorphismId>,
    right_eq: &Vec<MorphismId>,
) -> Option<Vec<MorphismId>> {
    let mut split = 0;
    let mut eq_index = 0;
    let mut current_i = 0;
    loop {
        if current_i == data.len() {
            // Failed to find a match
            return None;
        }
        let morphism = data[current_i];
        if morphism == left_eq[eq_index] {
            // Look for a match
            eq_index += 1;
            current_i += 1;
            if eq_index == left_eq.len() {
                break;
            }
        } else {
            // Start over
            eq_index = 0;
            split += 1;
            current_i = split;
        }
    }

    // Found a match in range split..current_i
    assert_eq!(left_eq.len(), current_i - split);
    Some(
        data[..split]
            .iter()
            .chain(right_eq)
            .chain(&data[current_i..])
            .cloned()
            .collect(),
    )
}
