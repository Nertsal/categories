use geng::prelude::{ApproxEq, Vec2};

use crate::{ForceParameters, PhysicsBody};

/// Updates the positions and velocities of vertices and edges
pub fn apply_forces<K: Clone + PartialEq, T: PhysicsBody>(
    parameters: &ForceParameters,
    delta_time: f32,
    bodies: &mut impl crate::Collection<Key = K, Item = T>,
    connections: &impl crate::Collection<Key = K, Item = Vec<K>>,
) {
    // Calculate forces
    let mut forces = Vec::with_capacity(bodies.len().unwrap_or(0));
    for (id, body) in bodies.iter() {
        let connected = connections.get(&id);
        let attract_force = connected
            .map(|others| {
                others
                    .iter()
                    .filter_map(|other| bodies.get(other))
                    .filter(|other| !body.is_vertex() || other.is_vertex())
                    .map(|other| attract_force(body, other, parameters))
                    .fold(Vec2::ZERO, |a, b| a + b)
            })
            .unwrap_or(Vec2::ZERO);

        let repel_force = bodies
            .iter()
            .filter(|(_, other)| !body.is_vertex() || other.is_vertex())
            .map(|(_, other)| repel_force(body, other, parameters))
            .fold(Vec2::ZERO, |a, b| a + b);

        let force = (attract_force + repel_force).clamp_len(..=parameters.force_max);
        forces.push((id, force));
    }

    // Apply forces & move
    // Vertices
    for (id, force) in forces {
        let body = bodies.get_mut(&id).unwrap();

        let velocity = body.get_velocity() + force / body.get_mass() * delta_time;
        let velocity = velocity * parameters.damping_factor;
        body.set_velocity(velocity);
        body.set_position(body.get_position() + velocity * delta_time * parameters.vertex_speed);
    }
    //Edges
}

fn attract_force(
    body: &impl PhysicsBody,
    other: &impl PhysicsBody,
    parameters: &ForceParameters,
) -> Vec2<f32> {
    let delta = other.get_position() - body.get_position();

    let distance = delta.len();
    if distance.approx_eq(&0.0) {
        return Vec2::ZERO;
    }

    let direction = delta / distance;
    let force = if body.is_vertex() {
        parameters.force_attract_vertex
    } else {
        parameters.force_attract_edge
    };
    let strength = 1.0 * force * distance * 0.5;
    direction * strength
}

fn repel_force(
    body: &impl PhysicsBody,
    other: &impl PhysicsBody,
    parameters: &ForceParameters,
) -> Vec2<f32> {
    let delta = body.get_position() - other.get_position();

    let distance = delta.len();
    if distance.approx_eq(&0.0) || distance > parameters.repel_distance_max {
        return Vec2::ZERO;
    }

    let direction = delta / distance;

    let distance_sqrd = distance * distance;
    let force = if body.is_vertex() {
        parameters.force_repel_vertex
    } else if other.is_vertex() {
        parameters.force_repel_edge_vertex
    } else {
        parameters.force_repel_edge
    };
    let strength = force * ((body.get_mass() * other.get_mass()) / distance_sqrd);
    direction * strength
}
