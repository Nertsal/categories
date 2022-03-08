use super::*;
use std::hash::Hash;

pub fn constraint_unordered<L: PartialEq, T: PartialEq>(
    constraints: impl IntoIterator<Item = (L, Option<T>)>,
    actual: impl IntoIterator<Item = T>,
) -> Option<impl Iterator<Item = (L, T)>> {
    let mut unknowns = Vec::new();
    let mut actual = actual.into_iter().collect::<Vec<_>>();

    for (constraint, value) in constraints {
        match value {
            Some(value) => match actual.iter().position(|actual| *actual == value) {
                Some(i) => {
                    actual.remove(i);
                }
                None => return None,
            },
            None => {
                unknowns.push(constraint);
            }
        }
    }

    if unknowns.len() != actual.len() {
        return None;
    }

    Some(unknowns.into_iter().zip(actual.into_iter()))
}

pub fn constraint_ordered<L: Hash + Eq, T: PartialEq>(
    constraints: impl IntoIterator<Item = (L, Option<T>)>,
    actuals: impl IntoIterator<Item = T>,
) -> Option<impl Iterator<Item = (L, T)>> {
    let mut binds = Vec::new();

    for ((constraint, value), actual) in constraints.into_iter().zip(actuals.into_iter()) {
        match value {
            Some(value) => {
                if value != actual {
                    return None;
                }
            }
            None => {
                binds.push((constraint, actual));
            }
        }
    }

    Some(binds.into_iter())
}

pub fn decompose_morphism<O, M, E>(
    morphism_id: MorphismId,
    category: &Category<O, M, E>,
) -> Vec<MorphismId> {
    fn decompose<O, M, E>(
        morphism_id: MorphismId,
        morphism: &Morphism<M>,
        category: &Category<O, M, E>,
    ) -> Vec<MorphismId> {
        morphism
            .tags
            .iter()
            .find_map(|tag| match tag {
                &MorphismTag::Composition { first, second } => {
                    category.morphisms.get(&first).and_then(|morphism_f| {
                        category.morphisms.get(&second).map(|morphism_g| {
                            let mut composition = decompose(first, morphism_f, category);
                            composition.extend(decompose(second, morphism_g, category));
                            composition
                        })
                    })
                }
                _ => None,
            })
            .unwrap_or_else(|| vec![morphism_id])
    }

    category
        .morphisms
        .get(&morphism_id)
        .map(|morphism| decompose(morphism_id, morphism, category))
        .expect("Morphism does not exist in the category")
}

#[test]
fn test_unordered() {
    let result: Vec<_> = constraint_unordered(vec![("A", Some(0)), ("B", Some(0))], vec![0, 0])
        .unwrap()
        .collect();
    assert!(result.is_empty());
}
