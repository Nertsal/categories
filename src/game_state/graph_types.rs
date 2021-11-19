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
    pub color: Color<f32>,
}

impl<T> Arrow<T> {
    pub fn new(
        label: &str,
        from: T,
        to: T,
        connection: ArrowConnection,
        color: Color<f32>,
    ) -> Self {
        Self {
            label: label.to_owned(),
            from,
            to,
            connection,
            color,
        }
    }
}

impl graphs::GraphEdge for Arrow<VertexId> {
    fn end_points(&self) -> [&graphs::VertexId; 2] {
        [&self.from, &self.to]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArrowConnection {
    Best,
    Regular,
    Unique,
}

impl ArrowConnection {
    pub fn color(&self) -> Color<f32> {
        match self {
            ArrowConnection::Best => ARROW_BEST_COLOR,
            ArrowConnection::Regular => ARROW_REGULAR_COLOR,
            ArrowConnection::Unique => ARROW_UNIQUE_COLOR,
        }
    }
}

impl<T: PartialEq> Arrow<T> {
    pub fn check_constraint(&self, constraint: &ArrowConstraint<T>) -> bool {
        self.from == constraint.from
            && self.to == constraint.to
            && self.connection.check_constraint(&constraint.connection)
    }
}

impl ArrowConnection {
    pub fn check_constraint(&self, constraint: &Self) -> bool {
        match (constraint, self) {
            (ArrowConnection::Regular, _) => true,
            (constraint, connection) => connection == constraint,
        }
    }
}
