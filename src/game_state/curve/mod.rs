use super::*;

mod cardinal;
mod parabola;

pub use cardinal::CardinalSpline;
pub use parabola::ParabolaCurve;

/// A trait representing a generic curve.
pub trait Curve {
    /// Converts a curve into a chain (a list of segments) for rendering and collision detection.
    fn chain(&self, resolution: usize, width: f32, color: Color<f32>) -> Chain;
}

/// A trait representing a [curve](https://en.wikipedia.org/wiki/Cubic_Hermite_spline#Cardinal_spline)
/// defined by intervals with key points and tangents.
pub trait CubicHermiteCurve {
    /// Get the intervals of the curve.
    fn intervals(&self) -> Vec<CurveInterval>;
}

impl<T: CubicHermiteCurve> Curve for T {
    fn chain(&self, resolution: usize, width: f32, color: Color<f32>) -> Chain {
        let intervals = self.intervals();
        let mut vertices = Vec::with_capacity(resolution * intervals.len());

        for interval in intervals {
            let step = 1. / resolution as f32;
            for i in 0..=resolution {
                let t = step * i as f32;
                vertices.push(interval.interpolate(t));
            }
        }

        Chain {
            vertices,
            width,
            color,
        }
    }
}

/// Represents a single interval of the curve.
#[derive(Debug)]
pub struct CurveInterval {
    pub point_start: Vec2<f32>,
    pub point_end: Vec2<f32>,
    pub tangent_start: Vec2<f32>,
    pub tangent_end: Vec2<f32>,
}

impl CurveInterval {
    /// Returns a point on the curve interval
    pub fn interpolate(&self, t: f32) -> Vec2<f32> {
        let p0 = self.point_start;
        let p1 = self.point_end;
        let m0 = self.tangent_start;
        let m1 = self.tangent_end;

        let t2 = t * t; // t^2
        let t3 = t2 * t; // t^3
        (2. * t3 - 3. * t2 + 1.) * p0
            + (t3 - 2. * t2 + t) * m0
            + (-2. * t3 + 3. * t2) * p1
            + (t3 - t2) * m1
    }
}
