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

    pub fn remove_vertex(&mut self, vertex_id: VertexId) {
        self.vertices.remove(&vertex_id);
        self.edges
            .retain(|_, edge| !edge.is_vertex_incident(vertex_id));
    }

    pub fn add_edge(&mut self, edge: E) -> Option<EdgeId> {
        let end_points = edge.end_points();
        if !self.vertices.contains(end_points[0]) || !self.vertices.contains(end_points[1]) {
            return None;
        }

        Some(self.edges.new_edge(edge))
    }
}
