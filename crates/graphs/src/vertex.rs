use std::collections::HashMap;

use super::*;

pub trait GraphVertex {}

impl<T> GraphVertex for T {}

pub struct Vertices<V: GraphVertex>(HashMap<VertexId, V>);

impl<V: GraphVertex> Vertices<V> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&VertexId, &V)> {
        self.0.iter()
    }

    pub fn insert(&mut self, id: VertexId, vertex: V) -> Option<V> {
        self.0.insert(id, vertex)
    }

    pub fn get(&self, id: &VertexId) -> Option<&V> {
        self.0.get(id)
    }

    pub fn get_mut(&mut self, id: &VertexId) -> Option<&mut V> {
        self.0.get_mut(id)
    }

    pub(super) fn remove(&mut self, id: &VertexId) -> Option<V> {
        self.0.remove(id)
    }

    pub fn contains(&self, id: &VertexId) -> bool {
        self.0.contains_key(id)
    }
}
