use super::*;

/// Simulation parameters.
/// Partially borrowed from https://docs.rs/force_graph
#[derive(Debug)]
pub struct ForceParameters {
    pub force_spring_vertex: f32,
    pub force_spring_edge: f32,
    pub force_charge_vertex: f32,
    pub force_charge_edge: f32,
    pub force_max: f32,
    pub vertex_speed: f32,
    pub damping_factor: f32,
    pub repel_distance_max: f32,
}

impl Default for ForceParameters {
    fn default() -> Self {
        Self {
            force_spring_vertex: 10.0,
            force_spring_edge: 10.0,
            force_charge_vertex: 1000.0,
            force_charge_edge: 100.0,
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
    pub bodies: Vec<ForceBody>,
    pub edge: E,
}

impl<E: GraphEdge> ForceEdge<E> {
    pub fn new(
        vertex_a: Vec2<f32>,
        vertex_b: Vec2<f32>,
        bodies_count: usize,
        mass: f32,
        edge: E,
    ) -> Self {
        let delta = vertex_b - vertex_a;
        let bodies = (0..bodies_count)
            .map(|i| ForceBody::new(delta * i as f32 / bodies_count as f32 + vertex_a, mass))
            .collect();
        Self { bodies, edge }
    }

    pub fn get_center_mut(&mut self) -> Option<&mut ForceBody> {
        let len = self.bodies.len();
        self.bodies.get_mut(len / 2)
    }
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
    pub fn new(position: Vec2<f32>, mass: f32) -> Self {
        Self {
            position,
            mass,
            velocity: Vec2::ZERO,
        }
    }

    pub(crate) fn attract_force(&self, other: &Self, force_spring: f32) -> Vec2<f32> {
        let delta = other.position - self.position;

        let distance = delta.len();
        if distance.approx_eq(&0.0) {
            return Vec2::ZERO;
        }

        let direction = delta / distance;

        let strength = 1.0 * force_spring * distance * 0.5;
        direction * strength
    }

    pub(crate) fn repel_force(
        &self,
        other: &Self,
        force_charge: f32,
        repel_distance_max: f32,
    ) -> Vec2<f32> {
        let delta = self.position - other.position;

        let distance = delta.len();
        if distance.approx_eq(&0.0) || distance > repel_distance_max {
            return Vec2::ZERO;
        }

        let direction = delta / distance;

        let distance_sqrd = distance * distance;
        let strength = force_charge * ((self.mass * other.mass) / distance_sqrd);
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
