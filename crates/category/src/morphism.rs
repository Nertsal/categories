use std::collections::HashMap;

use super::*;

#[derive(Debug, Clone)]
pub struct Morphism<T> {
    pub connection: MorphismConnection,
    pub tags: Vec<MorphismTag>,
    pub inner: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MorphismTag<O = ObjectId, M = MorphismId> {
    Identity(O),
    Unique,
    Composition { first: M, second: M },
    Isomorphism(M, M),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MorphismConnection<T = ObjectId> {
    Regular { from: T, to: T },
    Isomorphism(T, T),
}

impl MorphismConnection {
    /// Returns the ids of the connected object in an arbitrary order.
    pub fn end_points(&self) -> [&ObjectId; 2] {
        match self {
            MorphismConnection::Regular { from, to } => [from, to],
            MorphismConnection::Isomorphism(a, b) => [a, b],
        }
    }

    pub fn is_object_connected(&self, id: ObjectId) -> bool {
        self.end_points().iter().any(|&&object| object == id)
    }
}

pub struct Morphisms<T> {
    morphisms: HashMap<MorphismId, Morphism<T>>,
    next_id: MorphismId,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord)]
pub struct MorphismId(u64);

impl MorphismId {
    pub fn raw(&self) -> u64 {
        self.0
    }
}

impl<T> Morphisms<T> {
    pub fn new() -> Self {
        Self {
            morphisms: HashMap::new(),
            next_id: MorphismId(0),
        }
    }

    pub(crate) fn new_morphism(&mut self, morphism: Morphism<T>) -> MorphismId {
        let id = self.next_id;
        self.next_id.0 += 1;
        assert!(
            self.morphisms.insert(id, morphism).is_none(),
            "Failed to generate new edge"
        );
        id
    }

    pub(crate) fn insert(
        &mut self,
        morphism: Morphism<T>,
        id: MorphismId,
    ) -> Result<Option<Morphism<T>>, ()> {
        if id.0 >= self.next_id.0 {
            return Err(());
        }

        Ok(self.morphisms.insert(id, morphism))
    }

    pub fn len(&self) -> usize {
        self.morphisms.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&MorphismId, &Morphism<T>)> {
        self.morphisms.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&MorphismId, &mut Morphism<T>)> {
        self.morphisms.iter_mut()
    }

    pub fn remove(&mut self, id: &MorphismId) -> Option<Morphism<T>> {
        self.morphisms.remove(id)
    }

    pub fn retain(&mut self, f: impl FnMut(&MorphismId, &mut Morphism<T>) -> bool) {
        self.morphisms.retain(f);
    }

    pub fn get(&self, id: &MorphismId) -> Option<&Morphism<T>> {
        self.morphisms.get(id)
    }

    pub fn get_mut(&mut self, id: &MorphismId) -> Option<&mut Morphism<T>> {
        self.morphisms.get_mut(id)
    }

    pub fn contains(&self, id: &MorphismId) -> bool {
        self.morphisms.contains_key(id)
    }
}
