use super::*;

pub type Graph = force_graph::ForceGraph<Point, Arrow<VertexId>>;
pub type Vertex = ForceVertex<Point>;
pub type Edge = ForceEdge<Arrow<VertexId>>;

#[derive(Debug, Clone)]
pub struct Point {
    pub label: String,
    pub radius: f32,
    pub color: Color<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arrow<T> {
    pub label: String,
    pub from: T,
    pub to: T,
    pub connection: ArrowConnection,
}

impl<T> Arrow<T> {
    pub fn color(&self) -> Color<f32> {
        match self.connection {
            ArrowConnection::Best => ARROW_BEST_COLOR,
            ArrowConnection::Regular => ARROW_REGULAR_COLOR,
            ArrowConnection::Unique => ARROW_UNIQUE_COLOR,
        }
    }
}

impl graphs::GraphEdge for Arrow<VertexId> {
    fn end_points(&self) -> [&graphs::VertexId; 2] {
        [&self.from, &self.to]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArrowConnection {
    Best,
    Regular,
    Unique,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrowConstraint<T> {
    pub from: T,
    pub to: T,
    pub connection: ArrowConnection,
}

impl<T: PartialEq> Arrow<T> {
    pub fn check_constraint(&self, constraint: &ArrowConstraint<T>) -> bool {
        self.from == constraint.from && self.to == constraint.to
    }
}
