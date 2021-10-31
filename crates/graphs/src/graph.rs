use super::*;

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

    /// Removes the vertex and connected edges from the graph.
    pub fn remove_vertex(&mut self, vertex_id: VertexId) {
        self.vertices.remove(&vertex_id);
        self.edges
            .retain(|_, edge| !edge.is_vertex_incident(vertex_id));
    }

    /// Removes the edge from the graph.
    pub fn remove_edge(&mut self, edge_id: EdgeId) {
        self.edges.remove(&edge_id);
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
