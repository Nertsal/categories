use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GraphObject {
    Vertex { id: VertexId },
    Edge { id: EdgeId },
}

impl GraphObject {
    pub fn vertex(&self) -> Option<&VertexId> {
        match self {
            Self::Vertex { id } => Some(id),
            _ => None,
        }
    }

    pub fn edge(&self) -> Option<&EdgeId> {
        match self {
            Self::Edge { id } => Some(id),
            _ => None,
        }
    }
}

pub struct Graph<V: GraphVertex, E: GraphEdge> {
    pub vertices: Vertices<V>,
    pub edges: Edges<E>,
}

impl<V: GraphVertex, E: GraphEdge> Graph<V, E> {
    pub fn new() -> Self {
        Self {
            vertices: Vertices::new(),
            edges: Edges::new(),
        }
    }

    pub fn new_vertex(&mut self, vertex: V) -> VertexId {
        self.vertices.new_vertex(vertex)
    }

    /// Adds a new edge to the graph.
    /// Returns None if the graph does not contain any of the vertices.
    pub fn new_edge(&mut self, edge: E) -> Option<EdgeId> {
        let end_points = edge.end_points();
        if !self.vertices.contains(end_points[0]) || !self.vertices.contains(end_points[1]) {
            return None;
        }
        Some(self.edges.new_edge(edge))
    }

    pub fn insert_vertex(&mut self, vertex: V, vertex_id: VertexId) -> Result<Option<V>, ()> {
        self.vertices.new_vertex_id(vertex, vertex_id)
    }

    pub fn insert_edge(&mut self, edge: E, edge_id: EdgeId) -> Result<Option<E>, ()> {
        self.edges.insert_edge(edge, edge_id)
    }

    /// Removes the vertex and connected edges from the graph.
    pub fn remove_vertex(&mut self, vertex_id: VertexId) -> (Option<V>, Vec<(EdgeId, E)>) {
        let vertex = self.vertices.remove(&vertex_id);

        let removes: Vec<_> = self
            .edges
            .iter()
            .filter(|(_, edge)| edge.is_vertex_incident(vertex_id))
            .map(|(&id, _)| id)
            .collect();
        let mut edges = Vec::new();
        for remove in removes {
            edges.push((remove, self.edges.remove(&remove).unwrap()));
        }

        (vertex, edges)
    }

    /// Removes the edge from the graph.
    pub fn remove_edge(&mut self, edge_id: EdgeId) -> Option<E> {
        self.edges.remove(&edge_id)
    }

    pub fn neighbours<'a>(&'a self, vertex: VertexId) -> impl Iterator<Item = VertexId> + 'a {
        self.edges.iter().filter_map(move |(_, edge)| {
            let endpoints = edge.end_points();
            if *endpoints[0] == vertex {
                Some(*endpoints[1])
            } else if *endpoints[1] == vertex {
                Some(*endpoints[0])
            } else {
                None
            }
        })
    }
}
