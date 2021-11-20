use std::collections::HashMap;

pub trait GraphVertex {}

impl<T> GraphVertex for T {}

pub struct Vertices<V: GraphVertex> {
    vertices: HashMap<VertexId, V>,
    next_id: VertexId,
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
pub struct VertexId(u64);

impl<V: GraphVertex> Vertices<V> {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            next_id: VertexId(0),
        }
    }

    pub(crate) fn new_vertex(&mut self, vertex: V) -> VertexId {
        let id = self.next_id;
        self.next_id.0 += 1;
        assert!(
            self.vertices.insert(id, vertex).is_none(),
            "Failed to generate new vertex"
        );
        id
    }

    pub(crate) fn new_vertex_id(
        &mut self,
        vertex: V,
        vertex_id: VertexId,
    ) -> Result<Option<V>, ()> {
        if vertex_id.0 >= self.next_id.0 {
            return Err(());
        }

        Ok(self.vertices.insert(vertex_id, vertex))
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&VertexId, &V)> {
        self.vertices.iter()
    }

    pub fn get(&self, id: &VertexId) -> Option<&V> {
        self.vertices.get(id)
    }

    pub fn get_mut(&mut self, id: &VertexId) -> Option<&mut V> {
        self.vertices.get_mut(id)
    }

    pub(crate) fn remove(&mut self, id: &VertexId) -> Option<V> {
        self.vertices.remove(id)
    }

    pub fn contains(&self, id: &VertexId) -> bool {
        self.vertices.contains_key(id)
    }
}
