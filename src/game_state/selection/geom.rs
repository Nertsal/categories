use super::*;

/// Calculate the distance from a point to a line segment
pub fn distance_point_segment(
    point: Vec2<f32>,
    segment_start: Vec2<f32>,
    segment_end: Vec2<f32>,
) -> f32 {
    // Project on the segment
    let delta = point - segment_start;
    let direction = segment_end - segment_start;
    let segment_len = direction.len();
    if segment_len < 1e-5 {
        // Segment is so small it resembles a point
        // Done to avoid division by 0
        return delta.len();
    }

    let direction_norm = direction / segment_len;
    let projection = Vec2::dot(delta, direction_norm);
    if projection < 0.0 {
        // The projection is outside of the line, closer to the start
        return delta.len();
    }
    if projection > segment_len {
        // The projection is outside of the line, closer to the end
        return (point - segment_end).len();
    }

    // Project on the normal
    let normal = direction_norm.rotate_90();
    let projection = Vec2::dot(delta, normal).abs();
    projection
}

/// Calculate whether the aabb and the segment overlap
pub fn overlap_aabb_segment(
    aabb: &AABB<f32>,
    segment_start: Vec2<f32>,
    segment_end: Vec2<f32>,
) -> bool {
    // Either one of segment points is inside
    // Or the segment intersects one of the edges
    if aabb.contains(segment_start) || aabb.contains(segment_end) {
        return true;
    }

    let top_left = aabb.top_left();
    let top_right = aabb.top_right();
    let bottom_left = aabb.bottom_left();
    let bottom_right = aabb.bottom_right();

    intersect_segment_segment(segment_start, segment_end, top_left, top_right).is_some()
        || intersect_segment_segment(segment_start, segment_end, bottom_left, bottom_right)
            .is_some()
        || intersect_segment_segment(segment_start, segment_end, bottom_right, top_right).is_some()
        || intersect_segment_segment(segment_start, segment_end, bottom_left, top_left).is_some()
}

/// Calculate the intersection point of two lines
pub fn intersect_segment_segment(
    segment_start: Vec2<f32>,
    segment_end: Vec2<f32>,
    other_start: Vec2<f32>,
    other_end: Vec2<f32>,
) -> Option<Vec2<f32>> {
    let segment_dir = segment_end - segment_start;
    let other_dir = other_end - other_start;

    fn cross(a: Vec2<f32>, b: Vec2<f32>) -> f32 {
        a.x * b.y - a.y * b.x
    }

    let start_delta = other_start - segment_start;
    let rxs = cross(segment_dir, other_dir);

    if rxs == 0.0 {
        // Collinear
        return None;
    }

    let tx = cross(start_delta, other_dir);
    let t = tx / rxs;
    let ux = cross(start_delta, segment_dir);
    let u = ux / rxs;

    if t < 0.0 || t > 1.0 || u < 0.0 || u > 1.0 {
        // No intersection
        return None;
    }

    // Intersection
    Some(segment_start + segment_dir * t)
}
