use super::*;

impl<V: GraphVertex, E: GraphEdge> ForceGraph<V, E> {
    /// Updates the positions of vertices and edges
    pub fn update(&mut self, delta_time: f32) {
        // Calculate forces
        // Vertices
        let mut force_vertices = Vec::with_capacity(self.graph.vertices.len());
        for (&id, vertex) in self
            .graph
            .vertices
            .iter()
            .filter(|(_, vertex)| !vertex.is_anchor)
        {
            let mut force = Vec2::ZERO;
            for neighbour in self.graph.neighbours(id) {
                force += vertex.body.attract_force(
                    &self.graph.vertices.get(&neighbour).unwrap().body,
                    self.parameters.force_spring_vertex,
                );
            }
            for (_, other) in self.graph.vertices.iter() {
                force += vertex.body.repel_force(
                    &other.body,
                    self.parameters.force_charge_vertex,
                    self.parameters.repel_distance_max,
                );
            }
            force_vertices.push((id, force.clamp(self.parameters.force_max)));
        }
        // Edges
        let mut force_edges = Vec::with_capacity(self.graph.edges.len());
        for (&id, edge) in self.graph.edges.iter() {
            let [p0, p1] = edge
                .end_points()
                .map(|vertex| &self.graph.vertices.get(vertex).unwrap().body);
            let bodies_count = edge.bodies.len();
            let mut bodies = Vec::with_capacity(bodies_count + 2);
            bodies.push(p0);
            bodies.extend(edge.bodies.iter());
            bodies.push(p1);

            let mut forces = Vec::with_capacity(bodies_count);
            for i in 1..=bodies_count {
                let mut force = Vec2::ZERO;

                force += bodies[i].attract_force(bodies[i - 1], self.parameters.force_spring_edge);
                force += bodies[i].attract_force(bodies[i + 1], self.parameters.force_spring_edge);

                for other in self
                    .graph
                    .edges
                    .iter()
                    .map(|(_, edge)| edge.bodies.iter())
                    .flatten()
                {
                    force += bodies[i].repel_force(
                        other,
                        self.parameters.force_charge_edge,
                        self.parameters.repel_distance_max,
                    );
                }
                for other in self.graph.vertices.iter().map(|(_, vertex)| &vertex.body) {
                    force += bodies[i].repel_force(
                        other,
                        self.parameters.force_charge_edge_vertex,
                        self.parameters.repel_distance_max,
                    );
                }

                forces.push(force.clamp(self.parameters.force_max));
            }

            force_edges.push((id, forces));
        }

        // Apply forces & move
        // Vertices
        for (id, force) in force_vertices {
            let vertex = self.graph.vertices.get_mut(&id).unwrap();
            vertex.body.update(force, delta_time, &self.parameters);
        }
        // Edges
        for (id, forces) in force_edges {
            let edge = self.graph.edges.get_mut(&id).unwrap();
            for (body, force) in edge.bodies.iter_mut().zip(forces) {
                body.update(force, delta_time, &self.parameters);
            }
        }
    }
}
