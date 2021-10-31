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
                    &self.parameters,
                );
            }
            for (_, other) in self.graph.vertices.iter() {
                force += vertex.body.repel_force(&other.body, &self.parameters);
            }
            force_vertices.push((id, force.clamp(self.parameters.force_max)));
        }
        // Edges
        let mut force_edges = Vec::with_capacity(self.graph.edges.len());
        for (&id, edge) in self.graph.edges.iter() {
            let mut force = Vec2::ZERO;
            for vertex in edge.end_points() {
                force += edge.body.attract_force(
                    &self.graph.vertices.get(vertex).unwrap().body,
                    &self.parameters,
                );
            }
            for (_, other) in self.graph.edges.iter() {
                force += edge.body.repel_force(&other.body, &self.parameters);
            }
            force_edges.push((id, force.clamp(self.parameters.force_max)));
        }

        // Apply forces & move
        // Vertices
        for (id, force) in force_vertices {
            let vertex = self.graph.vertices.get_mut(&id).unwrap();
            vertex.body.update(force, delta_time, &self.parameters);
        }
        // Edges
        for (id, force) in force_edges {
            let edge = self.graph.edges.get_mut(&id).unwrap();
            edge.body.update(force, delta_time, &self.parameters);
        }
    }
}
