use super::*;

/// Represents a parabola curve `f(t) = at^2 + bt + c`.
pub struct ParabolaCurve {
    equation: [Vec2<f32>; 3],
}

impl ParabolaCurve {
    /// Creates a new parabol passing through three points
    pub fn new(points: [Vec2<f32>; 3]) -> Self {
        let [p0, p1, p2] = points;
        // f(0) = p0 (start)    | c = p0
        // f(0.5) = p1 (middle) | 0.25 * a + 0.5 * b + c = p1
        // f(1) = p2 (end)      | a + b + c = p2
        let c = p0;
        let b = 4. * p1 - p2 - 3. * p0;
        let a = p2 - p0 - b;

        Self {
            equation: [a, b, c],
        }
    }

    /// Return the point on the parabola
    fn get(&self, t: f32) -> Vec2<f32> {
        let [a, b, c] = self.equation;
        a * t * t + b * t + c
    }
}

impl Curve for ParabolaCurve {
    fn chain(&self, resolution: usize, width: f32, color: Color<f32>) -> Chain {
        let mut vertices = Vec::with_capacity(resolution * 2);

        let step = 0.5 / resolution as f32;
        for i in 0..=resolution * 2 {
            let t = step * i as f32;
            vertices.push(self.get(t));
        }

        Chain {
            vertices,
            width,
            color,
        }
    }
}
