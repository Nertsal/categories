use std::hash::Hash;

use super::*;

/// Simulation parameters.
/// Partially borrowed from https://docs.rs/force_graph
#[derive(Debug)]
pub struct ForceParameters {
    pub force_attract_vertex: f32,
    pub force_attract_edge: f32,
    pub force_repel_vertex: f32,
    pub force_repel_edge: f32,
    pub force_repel_edge_vertex: f32,
    pub force_max: f32,
    pub vertex_speed: f32,
    pub damping_factor: f32,
    pub repel_distance_max: f32,
}

impl Default for ForceParameters {
    fn default() -> Self {
        Self {
            force_attract_vertex: 10.0,
            force_attract_edge: 10.0,
            force_repel_vertex: 500.0,
            force_repel_edge: 100.0,
            force_repel_edge_vertex: 20.0,
            force_max: 100.0,
            vertex_speed: 5.0,
            damping_factor: 0.95,
            repel_distance_max: 30.0,
        }
    }
}

pub trait PhysicsBody {
    /// Vertices are affected only by other vertices, while edges are affected by everything.
    fn is_vertex(&self) -> bool;
    fn get_mass(&self) -> f32;
    fn get_position(&self) -> Vec2<f32>;
    fn set_position(&mut self, position: Vec2<f32>);
    fn get_velocity(&self) -> Vec2<f32>;
    fn set_velocity(&mut self, velocity: Vec2<f32>);
}

pub trait Collection {
    type Key;
    type Item;
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (Self::Key, &Self::Item)> + 'a>;
    fn get(&self, key: &Self::Key) -> Option<&Self::Item>;
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Item>;
    fn len(&self) -> Option<usize> {
        None
    }
}

impl<K: Clone + Hash + Eq, V> Collection for std::collections::HashMap<K, V> {
    type Key = K;
    type Item = V;
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (Self::Key, &Self::Item)> + 'a> {
        Box::new(self.iter().map(|(key, item)| (key.clone(), item)))
    }
    fn get(&self, key: &Self::Key) -> Option<&Self::Item> {
        self.get(key)
    }
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Item> {
        std::collections::HashMap::get_mut(self, key)
    }
}

impl<T> Collection for Vec<T> {
    type Key = usize;
    type Item = T;
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (Self::Key, &Self::Item)> + 'a> {
        Box::new(self.as_slice().iter().enumerate())
    }
    fn get(&self, key: &Self::Key) -> Option<&Self::Item> {
        self.as_slice().get(*key)
    }
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Item> {
        self.as_mut_slice().get_mut(*key)
    }
}
