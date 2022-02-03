use super::*;

pub fn objects_under_point(
    category: &CategoryWrapper,
    position: Vec2<f32>,
) -> impl Iterator<Item = (&ObjectId, &Point)> {
    category
        .objects
        .iter()
        .filter(move |(_, object)| (object.position - position).len() <= object.radius)
}

pub fn morphisms_under_point(
    category: &CategoryWrapper,
    position: Vec2<f32>,
) -> impl Iterator<Item = (&MorphismId, &Arrow)> {
    morphisms_points(category)
        .filter(move |(_, _, points)| {
            points
                .iter()
                .zip(points.iter().skip(1))
                .any(|(&start, &end)| {
                    distance_point_segment(position, start, end) <= ARROW_WIDTH + SELECTION_RADIUS
                })
        })
        .map(|(id, edge, _)| (id, edge))
}

fn morphisms_points(
    category: &CategoryWrapper,
) -> impl Iterator<Item = (&MorphismId, &Arrow, Vec<Vec2<f32>>)> {
    category.morphisms.iter().filter_map(|(id, morphism)| {
        let [object_a, object_b] = category
            .inner
            .morphisms
            .get(id)
            .unwrap() // TODO: better error handling?
            .connection
            .end_points()
            .map(|id| category.objects.get(id));

        object_a
            .and_then(|object_a| {
                object_b
                    .map(|object_b| (object_a.position, object_b.position))
                    .map(|(pos_a, pos_b)| {
                        let mut points = Vec::with_capacity(morphism.positions.len() + 2);
                        points.push(pos_a);
                        points.extend(morphism.positions.iter().copied());
                        points.push(pos_b);
                        points
                    })
            })
            .map(|points| (id, morphism, points))
    })
}
