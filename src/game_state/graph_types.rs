use super::*;

pub type Graph = force_graph::ForceGraph<Point<VertexId>, Arrow<VertexId, EdgeId>>;
pub type Vertex = ForceVertex<Point<VertexId>>;
pub type Edge = ForceEdge<Arrow<VertexId, EdgeId>>;

#[derive(Debug, Clone)]
pub struct Point<O> {
    pub label: Label,
    pub radius: f32,
    pub tags: Vec<ObjectTag<Option<O>>>,
    pub color: Color<f32>,
}

#[derive(Debug, Clone)]
pub struct Arrow<O, M> {
    pub label: Label,
    pub from: O,
    pub to: O,
    pub tags: Vec<MorphismTag<Option<O>, Option<M>>>,
    pub color: Color<f32>,
}

impl graphs::GraphEdge for Arrow<VertexId, EdgeId> {
    fn end_points(&self) -> [&graphs::VertexId; 2] {
        [&self.from, &self.to]
    }
}
