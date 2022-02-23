use super::*;
use std::{collections::HashSet, hash::Hash};

pub fn constraint_unordered<L: Hash + Eq, T: Hash + Eq>(
    constraints: impl IntoIterator<Item = (L, Option<T>)>,
    actual: impl IntoIterator<Item = T>,
) -> Option<impl Iterator<Item = (L, T)>> {
    let mut unknowns = Vec::new();
    let mut actual = actual.into_iter().collect::<HashSet<_>>();

    for (constraint, value) in constraints {
        match value {
            Some(value) => {
                if !actual.remove(&value) {
                    return None;
                }
            }
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

pub(super) fn decompose_morphism<O, M>(
    morphism_id: MorphismId,
    morphism: &Morphism<M>,
    category: &Category<O, M>,
) -> Vec<MorphismId> {
    morphism
        .tags
        .iter()
        .find_map(|tag| match tag {
            &MorphismTag::Composition { first, second } => {
                category.morphisms.get(&first).and_then(|morphism_f| {
                    category.morphisms.get(&second).map(|morphism_g| {
                        let mut composition = decompose_morphism(first, morphism_f, category);
                        composition.extend(decompose_morphism(second, morphism_g, category));
                        composition
                    })
                })
            }
            _ => None,
        })
        .unwrap_or_else(|| vec![morphism_id])
}
