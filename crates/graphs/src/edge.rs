use std::collections::HashMap;

use super::*;

pub trait GraphEdge: PartialEq {
    fn end_points(&self) -> [&VertexId; 2];

    fn is_vertex_incident(&self, vertex: VertexId) -> bool {
        let end_points = self.end_points();
        *end_points[0] == vertex || *end_points[1] == vertex
    }
}

pub struct Edges<E: GraphEdge> {
    edges: HashMap<EdgeId, E>,
    next_id: EdgeId,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct EdgeId(u64);

impl<E: GraphEdge> Edges<E> {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            next_id: EdgeId(0),
        }
    }

    pub(crate) fn new_edge(&mut self, edge: E) -> EdgeId {
        let id = self.next_id;
        self.next_id.0 += 1;
        assert!(
            self.edges.insert(id, edge).is_none(),
            "Failed to generate new edge"
        );
        id
    }

    pub fn iter(&self) -> impl Iterator<Item = (&EdgeId, &E)> {
        self.edges.iter()
    }

    pub fn remove(&mut self, id: &EdgeId) -> Option<E> {
        self.edges.remove(id)
    }

    pub fn retain(&mut self, f: impl FnMut(&EdgeId, &mut E) -> bool) {
        self.edges.retain(f);
    }

    pub fn get(&self, id: &EdgeId) -> Option<&E> {
        self.edges.get(id)
    }

    pub fn get_mut(&mut self, id: &EdgeId) -> Option<&mut E> {
        self.edges.get_mut(id)
    }

    pub fn contains(&self, id: &EdgeId) -> bool {
        self.edges.contains_key(id)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Edge {
    pub v1: VertexId,
    pub v2: VertexId,
}

impl Edge {
    pub fn new(v1: VertexId, v2: VertexId) -> Self {
        Self { v1, v2 }
    }
}

impl GraphEdge for Edge {
    fn end_points(&self) -> [&VertexId; 2] {
        [&self.v1, &self.v2]
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DirectedEdge {
    pub from: VertexId,
    pub to: VertexId,
}

impl DirectedEdge {
    pub fn new(from: VertexId, to: VertexId) -> Self {
        Self { from, to }
    }
}

impl GraphEdge for DirectedEdge {
    fn end_points(&self) -> [&VertexId; 2] {
        [&self.from, &self.to]
    }
}
