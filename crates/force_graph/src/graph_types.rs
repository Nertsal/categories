use super::*;

/// Simulation parameters.
/// Partially borrowed from https://docs.rs/force_graph
#[derive(Debug)]
pub struct ForceParameters {
    pub force_spring: f32,
    pub force_charge: f32,
    pub force_max: f32,
    pub vertex_speed: f32,
    pub damping_factor: f32,
    pub repel_distance_max: f32,
}

impl Default for ForceParameters {
    fn default() -> Self {
        Self {
            force_charge: 1000.0,
            force_spring: 10.0,
            force_max: 280.0,
            vertex_speed: 5.0,
            damping_factor: 0.95,
            repel_distance_max: 30.0,
        }
    }
}

/// Handles positioning vertices and edges in the graph
/// for (better than none) displaying.
pub struct ForceGraph<V: GraphVertex, E: GraphEdge> {
    pub parameters: ForceParameters,
    pub graph: Graph<ForceVertex<V>, ForceEdge<E>>,
}

impl<V: GraphVertex, E: GraphEdge> ForceGraph<V, E> {
    pub fn new(parameters: ForceParameters) -> Self {
        Self {
            parameters,
            graph: Graph::new(),
        }
    }
}

/// A vertex in the ForceGraph.
#[derive(Debug, Clone)]
pub struct ForceVertex<V: GraphVertex> {
    pub is_anchor: bool,
    pub body: ForceBody,
    pub vertex: V,
}

/// An edge in the ForceGraph.
#[derive(Debug, Clone, PartialEq)]
pub struct ForceEdge<E: GraphEdge> {
    pub body: ForceBody,
    pub edge: E,
}

impl<E: GraphEdge> GraphEdge for ForceEdge<E> {
    fn end_points(&self) -> [&graphs::VertexId; 2] {
        self.edge.end_points()
    }
}

/// Represents a body in the graph affected by other bodies.
#[derive(Debug, Clone, PartialEq)]
pub struct ForceBody {
    pub position: Vec2<f32>,
    pub mass: f32,
    pub velocity: Vec2<f32>,
}

impl ForceBody {
    pub(crate) fn attract_force(&self, other: &Self, parameters: &ForceParameters) -> Vec2<f32> {
        let delta = other.position - self.position;

        let distance = delta.len();
        if distance.approx_eq(&0.0) {
            return Vec2::ZERO;
        }

        let direction = delta / distance;

        let strength = 1.0 * parameters.force_spring * distance * 0.5;
        direction * strength
    }

    pub(crate) fn repel_force(&self, other: &Self, parameters: &ForceParameters) -> Vec2<f32> {
        let delta = self.position - other.position;

        let distance = delta.len();
        if distance.approx_eq(&0.0) || distance > parameters.repel_distance_max {
            return Vec2::ZERO;
        }

        let direction = delta / distance;

        let distance_sqrd = distance * distance;
        let strength = parameters.force_charge * ((self.mass * other.mass) / distance_sqrd);
        direction * strength
    }

    pub(crate) fn update(
        &mut self,
        force: Vec2<f32>,
        delta_time: f32,
        parameters: &ForceParameters,
    ) {
        self.velocity += force / self.mass * delta_time;
        self.velocity *= parameters.damping_factor;
        self.position += self.velocity * delta_time * parameters.vertex_speed;
    }
}
