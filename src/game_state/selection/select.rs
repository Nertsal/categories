use super::*;

impl GameState {
    pub fn vertices_under_point(
        &self,
        position: Vec2<f32>,
    ) -> impl Iterator<Item = (&VertexId, &Vertex)> {
        self.force_graph
            .graph
            .vertices
            .iter()
            .filter(move |(_, vertex)| {
                (vertex.body.position - position).len() <= vertex.vertex.radius
            })
    }

    pub fn edges_under_point(&self, position: Vec2<f32>) -> impl Iterator<Item = (&EdgeId, &Edge)> {
        self.edges_points()
            .filter(move |(_, _, points)| {
                points
                    .iter()
                    .zip(points.iter().skip(1))
                    .any(|(&start, &end)| {
                        distance_point_segment(position, start, end)
                            <= ARROW_WIDTH + SELECTION_RADIUS
                    })
            })
            .map(|(id, edge, _)| (id, edge))
    }

    pub fn vertices_in_area(&self, area: AABB<f32>) -> impl Iterator<Item = (&VertexId, &Vertex)> {
        self.force_graph
            .graph
            .vertices
            .iter()
            .filter(move |(_, vertex)| {
                vertex.vertex.distance_to_aabb(vertex.body.position, &area) <= 0.0
            })
    }

    pub fn edges_in_area(&self, area: AABB<f32>) -> impl Iterator<Item = (&EdgeId, &Edge)> {
        self.edges_points()
            .filter(move |(_, _, points)| {
                points
                    .iter()
                    .zip(points.iter().skip(1))
                    .any(|(&start, &end)| overlap_aabb_segment(&area, start, end))
            })
            .map(|(id, edge, _)| (id, edge))
    }

    fn edges_points(&self) -> impl Iterator<Item = (&EdgeId, &Edge, Vec<Vec2<f32>>)> {
        self.force_graph
            .graph
            .edges
            .iter()
            .filter_map(|(id, edge)| {
                self.force_graph
                    .graph
                    .vertices
                    .get(&edge.edge.from)
                    .map(|vertex| vertex.body.position)
                    .and_then(|arrow_start| {
                        self.force_graph
                            .graph
                            .vertices
                            .get(&edge.edge.to)
                            .map(|vertex| (arrow_start, vertex.body.position))
                    })
                    .map(|(arrow_start, arrow_end)| {
                        let mut points = Vec::with_capacity(edge.bodies.len() + 2);
                        points.push(arrow_start);
                        points.extend(edge.bodies.iter().map(|body| body.position));
                        points.push(arrow_end);
                        points
                    })
                    .map(|points| (id, edge, points))
            })
    }
}
