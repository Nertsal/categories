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