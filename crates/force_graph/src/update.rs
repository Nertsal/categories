use super::*;

/// Updates the positions and velocities of vertices and edges
pub fn apply_forces<K: Clone, T: PhysicsBody>(
    parameters: &ForceParameters,
    delta_time: f32,
    bodies: &mut impl Collection<K, T>,
    attracts: &impl Collection<K, Vec<K>>,
    repels: &impl Collection<K, Vec<K>>,
) {
    // Calculate forces
    let mut forces = Vec::with_capacity(bodies.len().unwrap_or(0));
    for (id, body) in bodies.iter() {
        let attract_force = attracts
            .get(&id)
            .map(|others| {
                others
                    .iter()
                    .filter_map(|other| bodies.get(other))
                    .map(|other| attract_force(body, other, parameters.force_spring))
                    .fold(Vec2::ZERO, |a, b| a + b)
            })
            .unwrap_or(Vec2::ZERO);

        let repel_force = repels
            .get(&id)
            .map(|others| {
                others
                    .iter()
                    .filter_map(|other| bodies.get(other))
                    .map(|other| {
                        repel_force(
                            body,
                            other,
                            parameters.force_charge,
                            parameters.repel_distance_max,
                        )
                    })
                    .fold(Vec2::ZERO, |a, b| a + b)
            })
            .unwrap_or(Vec2::ZERO);

        let force = (attract_force + repel_force).clamp_len(..=parameters.force_max);
        forces.push((id, force));
    }

    // Apply forces & move
    for (id, force) in forces {
        let body = bodies.get_mut(&id).unwrap();

        let velocity = body.get_velocity() + force / body.get_mass() * delta_time;
        let velocity = velocity * parameters.damping_factor;
        body.set_velocity(velocity);
        body.set_position(body.get_position() + velocity * delta_time * parameters.vertex_speed);
    }
}

fn attract_force(
    body: &impl PhysicsBody,
    other: &impl PhysicsBody,
    force_spring: f32,
) -> Vec2<f32> {
    let delta = other.get_position() - body.get_position();

    let distance = delta.len();
    if distance.approx_eq(&0.0) {
        return Vec2::ZERO;
    }

    let direction = delta / distance;
    let strength = 1.0 * force_spring * distance * 0.5;
    direction * strength
}

fn repel_force(
    body: &impl PhysicsBody,
    other: &impl PhysicsBody,
    force_charge: f32,
    repel_distance_max: f32,
) -> Vec2<f32> {
    let delta = body.get_position() - other.get_position();

    let distance = delta.len();
    if distance.approx_eq(&0.0) || distance > repel_distance_max {
        return Vec2::ZERO;
    }

    let direction = delta / distance;

    let distance_sqrd = distance * distance;
    let strength = force_charge * ((body.get_mass() * other.get_mass()) / distance_sqrd);
    direction * strength
}
