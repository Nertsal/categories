use super::*;

pub struct Chain {
    pub vertices: Vec<Vec2<f32>>,
    pub width: f32,
}

impl Chain {
    pub fn segments(self) -> Vec<Segment> {
        let length = self.vertices.len();
        if length < 2 {
            return vec![];
        }

        let mut segments = Vec::with_capacity(length - 1);
        let mut prev = self.vertices[0];
        for &vertex in self.vertices.iter().skip(1) {
            segments.push(Segment {
                start: prev,
                end: vertex,
                width: self.width,
            });
            prev = vertex;
        }
        segments
    }

    pub fn triangle_strip(&self) -> Vec<Vec2<f32>> {
        let length = self.vertices.len();
        if length < 2 {
            return vec![];
        }

        let length = length * 2;
        let mut polygon = Vec::with_capacity(length);
        let mut prev = self.vertices[0];
        for vertex in self
            .vertices
            .iter()
            .skip(1)
            .copied()
            .chain(std::iter::once(prev))
        {
            let normal = (vertex - prev).rotate_90();
            let len = normal.len();
            let normal = if len.approx_eq(&0.0) {
                Vec2::ZERO
            } else {
                normal / len
            };
            let shift = normal * self.width / 2.0;
            polygon.push(prev + shift);
            polygon.push(prev - shift);
            prev = vertex;
        }
        polygon.to_vec()
    }
}

pub struct Segment {
    pub start: Vec2<f32>,
    pub end: Vec2<f32>,
    pub width: f32,
}

impl From<Segment> for Chain {
    fn from(segment: Segment) -> Self {
        Self {
            vertices: vec![segment.start, segment.end],
            width: segment.width,
        }
    }
}
