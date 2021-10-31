use super::*;

pub type Graph = graphs::Graph<Point, Arrow>;

#[derive(Debug, Clone)]
pub struct Point {
    pub position: Vec2<f32>,
    pub radius: f32,
    pub color: Color<f32>,
}

impl Point {
    pub fn distance_to_aabb(&self, aabb: &AABB<f32>) -> f32 {
        let dx = (aabb.x_min - self.position.x - self.radius)
            .max(self.position.x - self.radius - aabb.x_max);
        let dy = (aabb.y_min - self.position.y - self.radius)
            .max(self.position.y - self.radius - aabb.y_max);
        dx.max(dy)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arrow {
    pub from: graphs::VertexId,
    pub to: graphs::VertexId,
    pub color: Color<f32>,
    pub width: f32,
    pub connection: ArrowConnection,
}

impl graphs::GraphEdge for Arrow {
    fn end_points(&self) -> [&graphs::VertexId; 2] {
        [&self.from, &self.to]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArrowConnection {
    Solid,
    Dashed,
}
