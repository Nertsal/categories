use super::*;

pub struct Segment {
    pub start: Vec2<f32>,
    pub end: Vec2<f32>,
    pub width: f32,
}

impl Segment {
    pub fn polygon(&self) -> Vec<Vec2<f32>> {
        let normal = (self.end - self.start).rotate_90();
        let len = normal.len();
        let normal = if len < 1e-5 { Vec2::ZERO } else { normal / len };

        let shift = normal * self.width / 2.0;
        vec![
            self.start + shift,
            self.end + shift,
            self.end - shift,
            self.start - shift,
        ]
    }
}
