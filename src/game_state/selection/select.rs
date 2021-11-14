use super::*;

impl GameState {
    pub fn vertices_under_point(
        graph: &Graph,
        position: Vec2<f32>,
    ) -> impl Iterator<Item = (&VertexId, &Vertex)> {
        graph.graph.vertices.iter().filter(move |(_, vertex)| {
            (vertex.body.position - position).len() <= vertex.vertex.radius
        })
    }

    pub fn edges_under_point(
        graph: &Graph,
        position: Vec2<f32>,
    ) -> impl Iterator<Item = (&EdgeId, &Edge)> {
        Self::edges_points(graph)
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

    fn edges_points(graph: &Graph) -> impl Iterator<Item = (&EdgeId, &Edge, Vec<Vec2<f32>>)> {
        graph.graph.edges.iter().filter_map(|(id, edge)| {
            graph
                .graph
                .vertices
                .get(&edge.edge.from)
                .map(|vertex| vertex.body.position)
                .and_then(|arrow_start| {
                    graph
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
