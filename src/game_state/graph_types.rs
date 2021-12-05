use super::*;

pub type Graph = force_graph::ForceGraph<Point<VertexId>, Arrow<VertexId, EdgeId>>;
pub type Vertex = ForceVertex<Point<VertexId>>;
pub type Edge = ForceEdge<Arrow<VertexId, EdgeId>>;

#[derive(Debug, Clone)]
pub struct Point<O> {
    pub label: String,
    pub radius: f32,
    pub tags: Vec<ObjectTag<O>>,
    pub color: Color<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arrow<O, M> {
    pub label: Label,
    pub from: O,
    pub to: O,
    pub tags: Vec<MorphismTag<O, M>>,
    pub color: Color<f32>,
}

impl<O, M> Arrow<O, M> {
    pub fn new(
        label: &str,
        from: O,
        to: O,
        tags: Vec<MorphismTag<O, M>>,
        color: Color<f32>,
    ) -> Self {
        Self {
            label: label.to_owned(),
            from,
            to,
            tags,
            color,
        }
    }
}

impl graphs::GraphEdge for Arrow<VertexId, EdgeId> {
    fn end_points(&self) -> [&graphs::VertexId; 2] {
        [&self.from, &self.to]
    }
}
