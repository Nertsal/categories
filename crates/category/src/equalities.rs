use std::collections::HashSet;

use super::*;

pub struct Equalities {
    inner: HashSet<Equality>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Equality<T = MorphismId> {
    left: Vec<T>,
    right: Vec<T>,
}

impl<T> Equality<T> {
    pub fn new(left: Vec<T>, right: Vec<T>) -> Result<Self, ()> {
        // TODO: check validity
        if left.len() == 0 || right.len() == 0 {
            return Err(());
        }

        Ok(Self { left, right })
    }

    pub fn left(&self) -> &Vec<T> {
        &self.left
    }

    pub fn right(&self) -> &Vec<T> {
        &self.right
    }
}

impl Equalities {
    pub fn new() -> Self {
        Self {
            inner: HashSet::new(),
        }
    }

    pub fn new_equality(&mut self, equality: Equality) {
        self.inner.insert(equality);
    }

    pub fn contains_equality(&self, equality: &Equality) -> bool {
        self.inner.contains(equality)
    }

    pub fn remove_equality(&mut self, equality: &Equality) -> bool {
        self.inner.remove(equality)
    }

    pub fn all_equalities<'a>(&'a self) -> impl Iterator<Item = &'a Equality> + 'a {
        self.inner.iter()
    }

    pub fn get_equalities<'a>(
        &'a self,
        morphism: MorphismId,
    ) -> impl Iterator<Item = &'a Vec<MorphismId>> + 'a {
        self.inner.iter().filter_map(move |equality| {
            if equality.left.len() == 1 && equality.left[0] == morphism {
                Some(&equality.right)
            } else if equality.right.len() == 1 && equality.right[0] == morphism {
                Some(&equality.left)
            } else {
                None
            }
        })
    }

    pub fn get_equalities_with<'a>(
        &'a self,
        morphism: MorphismId,
    ) -> impl Iterator<Item = &'a Equality> + 'a {
        self.inner.iter().filter(move |equality| {
            equality.left.contains(&morphism) || equality.right.contains(&morphism)
        })
    }
}
