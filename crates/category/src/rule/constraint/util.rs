use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub fn constraint_unordered<L: Hash + Eq, T: Hash + Eq>(
    constraints: impl IntoIterator<Item = (L, Option<T>)>,
    actual: impl IntoIterator<Item = T>,
) -> Option<HashMap<L, T>> {
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

    Some(unknowns.into_iter().zip(actual.into_iter()).collect())
}
