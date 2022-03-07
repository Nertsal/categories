use std::collections::HashMap;

use super::*;

pub struct Equalities<T> {
    inner: HashMap<Equality, T>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Equality<M = MorphismId> {
    left: Vec<M>,
    right: Vec<M>,
}

impl<M> Equality<M> {
    /// Constructs a new equality and check its validity.
    /// May change the order (i.e. right may become left)
    /// to preserve equality uniqueness
    pub fn new(mut left: Vec<M>, mut right: Vec<M>) -> Result<Self, ()>
    where
        M: Ord,
    {
        // TODO: check validity
        if left.len() == 0 || right.len() == 0 {
            return Err(());
        }

        if left.len() < right.len() || left.len() == right.len() && left > right {
            std::mem::swap(&mut left, &mut right);
        }

        Ok(Self { left, right })
    }

    pub fn destructure(self) -> (Vec<M>, Vec<M>) {
        (self.left, self.right)
    }

    pub fn left(&self) -> &Vec<M> {
        &self.left
    }

    pub fn right(&self) -> &Vec<M> {
        &self.right
    }
}

impl<T> Equalities<T> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn new_equality(&mut self, equality: Equality, inner: T) {
        self.inner.insert(equality, inner);
    }

    pub fn contains_equality(&self, equality: &Equality) -> bool {
        self.inner.contains_key(equality)
    }

    pub fn remove_equality(&mut self, equality: &Equality) -> Option<T> {
        self.inner.remove(equality)
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a Equality, &'a T)> + 'a {
        self.inner.iter()
    }

    pub fn iter_equalities<'a>(&'a self) -> impl Iterator<Item = &'a Equality> + 'a {
        self.inner.keys()
    }

    pub fn iter_inners<'a>(&'a self) -> impl Iterator<Item = &'a T> + 'a {
        self.inner.values()
    }

    pub fn get_equalities<'a>(
        &'a self,
        morphism: MorphismId,
    ) -> impl Iterator<Item = &'a Vec<MorphismId>> + 'a {
        self.inner.keys().filter_map(move |equality| {
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
        self.inner.keys().filter(move |equality| {
            equality.left.contains(&morphism) || equality.right.contains(&morphism)
        })
    }
}
