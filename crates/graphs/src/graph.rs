use super::*;

pub struct Graph<V: GraphVertex, E: GraphEdge> {
    pub vertices: Vertices<V>,
    pub edges: Edges<E>,
    next_id: VertexId,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct VertexId(u64);

impl<V: GraphVertex, E: GraphEdge> Graph<V, E> {
    pub fn new() -> Self {
        Self {
            vertices: Vertices::new(),
            edges: Edges::new(),
            next_id: VertexId(0),
        }
    }

    pub fn new_vertex(&mut self, vertex: V) -> VertexId {
        let id = self.next_id;
        self.next_id = VertexId(id.0 + 1);
        assert!(
            self.vertices.insert(id, vertex).is_none(),
            "Failed to generate new vertex, id = {:?}",
            id
        );
        id
    }

    pub fn remove_vertex(&mut self, vertex_id: VertexId) {
        self.vertices.remove(&vertex_id);
        self.edges
            .retain(|edge| !edge.is_vertex_incident(vertex_id));
    }

    pub fn add_edge(&mut self, edge: E) -> Option<E> {
        let end_points = edge.end_points();
        if !self.vertices.contains(end_points[0]) || !self.vertices.contains(end_points[1]) {
            return Some(edge);
        }

        self.edges.add(edge);
        None
    }
}
