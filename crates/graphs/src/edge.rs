use super::*;

pub trait GraphEdge: PartialEq {
    fn end_points(&self) -> [&VertexId; 2];

    fn is_vertex_incident(&self, vertex: VertexId) -> bool {
        let end_points = self.end_points();
        *end_points[0] == vertex || *end_points[1] == vertex
    }
}

pub struct Edges<E: GraphEdge>(Vec<E>);

impl<E: GraphEdge> Edges<E> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn iter(&self) -> impl Iterator<Item = &E> {
        self.0.iter()
    }

    pub fn remove(&mut self, edge: &E) -> Option<E> {
        self.0
            .iter()
            .enumerate()
            .find(|&(_, other)| *other == *edge)
            .map(|(id, _)| id)
            .map(|id| self.0.remove(id))
    }

    pub fn retain(&mut self, f: impl Fn(&E) -> bool) {
        self.0.retain(f);
    }

    pub(super) fn add(&mut self, edge: E) {
        self.0.push(edge);
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
