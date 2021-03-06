use super::*;

pub fn targets_under_point(
    category: &Category,
    focused_category: FocusedCategory,
    local_pos: Vec2<f32>,
) -> Option<DragTarget> {
    selection::objects_under_point(category, local_pos)
        .next()
        .map(|(&id, _)| DragTarget::Object {
            id,
            category: focused_category,
        })
        .or_else(|| {
            // Drag edge
            selection::morphisms_under_point(&category, local_pos)
                .next()
                .map(|(&id, _)| DragTarget::Morphism {
                    id,
                    category: focused_category,
                })
        })
}

pub fn objects_under_point(
    category: &Category,
    local_pos: Vec2<f32>,
) -> impl Iterator<Item = (&ObjectId, &Object)> {
    category
        .objects
        .iter()
        .filter(move |(_, object)| (object.inner.position - local_pos).len() <= object.inner.radius)
}

pub fn morphisms_under_point(
    category: &Category,
    local_pos: Vec2<f32>,
) -> impl Iterator<Item = (&MorphismId, &Morphism)> {
    morphisms_points(category)
        .filter(move |(_, _, points)| {
            points
                .iter()
                .zip(points.iter().skip(1))
                .any(|(&start, &end)| {
                    distance_point_segment(local_pos, start, end) <= ARROW_WIDTH + SELECTION_RADIUS
                })
        })
        .map(|(id, edge, _)| (id, edge))
}

fn morphisms_points(
    category: &Category,
) -> impl Iterator<Item = (&MorphismId, &Morphism, Vec<Vec2<f32>>)> {
    category.morphisms.iter().filter_map(|(id, morphism)| {
        let [object_a, object_b] = morphism
            .connection
            .end_points()
            .map(|id| category.objects.get(id));

        object_a
            .and_then(|object_a| {
                object_b
                    .map(|object_b| (object_a.inner.position, object_b.inner.position))
                    .map(|(pos_a, pos_b)| {
                        let mut points = Vec::with_capacity(morphism.inner.positions.len() + 2);
                        points.push(pos_a);
                        points.extend(morphism.inner.positions.iter().copied());
                        points.push(pos_b);
                        points
                    })
            })
            .map(|points| (id, morphism, points))
    })
}
