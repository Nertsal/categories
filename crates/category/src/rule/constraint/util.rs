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
