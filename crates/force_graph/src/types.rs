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
            force_spring: 10.0,
            force_charge: 100.0,
            force_max: 100.0,
            vertex_speed: 5.0,
            damping_factor: 0.95,
            repel_distance_max: 30.0,
        }
    }
}

pub trait PhysicsBody {
    fn get_mass(&self) -> f32;
    fn get_position(&self) -> Vec2<f32>;
    fn set_position(&mut self, position: Vec2<f32>);
    fn get_velocity(&self) -> Vec2<f32>;
    fn set_velocity(&mut self, velocity: Vec2<f32>);
}

pub trait Collection<K, T> {
    fn iter(&self) -> Box<dyn Iterator<Item = (K, &T)>>;
    fn get(&self, key: &K) -> Option<&T>;
    fn get_mut(&mut self, key: &K) -> Option<&mut T>;
    fn len(&self) -> Option<usize> {
        None
    }
}
