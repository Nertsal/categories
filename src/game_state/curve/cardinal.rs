use super::*;

/// Represents a [cardinal spline](https://en.wikipedia.org/wiki/Cubic_Hermite_spline#Catmull%E2%80%93Rom_spline).
#[derive(Debug)]
pub struct CardinalSpline {
    /// The key points
    pub points: Vec<Vec2<f32>>,
    pub tension: f32,
}

impl CardinalSpline {
    /// Create a cardinal spline passing through points.
    /// Tension should be in range 0..=1.
    pub fn new(points: Vec<Vec2<f32>>, tension: f32) -> Self {
        Self { points, tension }
    }
}

impl CubicHermiteCurve for CardinalSpline {
    fn intervals(&self) -> Vec<CurveInterval> {
        // Tangents
        let len = self.points.len();
        let mut m = Vec::with_capacity(len);
        if len > 1 {
            m.push((
                0,
                (1. - self.tension) * (self.points[1] - self.points[0]) / (1. - 0.),
            ));
        }
        m.extend(
            self.points
                .iter()
                .zip(self.points.iter().skip(2))
                .map(|(&p0, &p2)| (1. - self.tension) * (p2 - p0) / (1. - 0.))
                .enumerate()
                .map(|(i, m)| (i + 1, m)),
        );
        if len > 2 {
            m.push((
                len - 1,
                (1. - self.tension) * (self.points[len - 1] - self.points[len - 2]) / (1. - 0.),
            ));
        }
        let mut m = m.into_iter();

        let (_, mut prev) = match m.next() {
            Some(first) => first,
            None => return Vec::new(),
        };

        let mut intervals = Vec::with_capacity(len - 1);
        while let Some((index, next)) = m.next() {
            intervals.push(CurveInterval {
                point_start: self.points[index - 1],
                point_end: self.points[index],
                tangent_start: prev,
                tangent_end: next,
            });
            prev = next;
        }

        intervals
    }
}
