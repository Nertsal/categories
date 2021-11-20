use super::*;

#[derive(Debug, Clone)]
pub struct Chain {
    pub vertices: Vec<Vec2<f32>>,
    pub width: f32,
    pub color: Color<f32>,
}

impl Chain {
    /// Returns the direction (not normalized) from the vertex before the last one to the last vertex.
    /// Returns None if there are less than 2 vertices
    pub fn end_direction(&self) -> Option<Vec2<f32>> {
        let length = self.vertices.len();
        if length < 2 {
            return None;
        }

        Some(self.vertices[length - 1] - self.vertices[length - 2])
    }

    pub fn length(&self) -> f32 {
        self.vertices
            .iter()
            .zip(self.vertices.iter().skip(1))
            .map(|(&a, &b)| (a - b).len())
            .sum()
    }

    /// Returns a part of the chain. The full chain's range is `0.0..=1.0`.
    ///
    /// # Examples
    /// ```
    /// let chain = Chain {
    ///     vertices: vec![vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0)],
    ///     width: 1.0,
    /// };
    /// assert_eq!(chain.clone().take_range_ratio(0.0..=1.0).vertices, chain.vertices);
    /// ```
    pub fn take_range_ratio(self, range: RangeInclusive<f32>) -> Self {
        let len = self.length();
        self.take_range_length(range.start() * len..=range.end() * len)
    }

    /// Returns a part of the chain. The full chain's range is `0.0..=chain.length()`.
    ///
    /// # Examples
    /// ```
    /// let chain = Chain {
    ///     vertices: vec![vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(1.0, 1.0), vec2(0.0, 1.0)],
    ///     width: 1.0,
    /// };
    /// assert_eq!(chain.clone().take_range_ratio(0.0..=chain.length()).vertices, chain.vertices);
    /// ```
    pub fn take_range_length(self, range: RangeInclusive<f32>) -> Self {
        let &(mut start_len) = range.start();
        let &(mut end_len) = range.end();

        let segments = self.vertices.iter().zip(self.vertices.iter().skip(1));

        let mut start = self.vertices[0];
        let mut start_i = 1;
        for (i, (&a, &b)) in segments.enumerate() {
            let len = (a - b).len();
            start_len -= len;

            if start_len < 0.0 {
                start = if len.approx_eq(&0.0) {
                    b
                } else {
                    b + (a - b) * (-start_len / len)
                };
                start_i = i + 1;
                break;
            }

            end_len -= len;
        }

        let mut vertices = vec![start];

        for i in start_i..self.vertices.len() {
            let a = self.vertices[i - 1];
            let b = self.vertices[i];
            let len = (a - b).len();
            end_len -= len;

            if end_len <= 0.0 {
                let end = if len.approx_eq(&0.0) {
                    b
                } else {
                    b + (a - b) * (-end_len / len)
                };
                vertices.push(end);
                break;
            }

            vertices.push(b);
        }

        Self { vertices, ..self }
    }

    pub fn segments(&self) -> Vec<Segment> {
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
                color: self.color,
            });
            prev = vertex;
        }
        segments
    }

    pub fn triangle_strip(&self) -> Vec<Vec2<f32>> {
        let len = self.vertices.len();
        if len < 2 {
            return vec![];
        }

        let mut polygon = Vec::with_capacity(len * 2);

        fn add(polygon: &mut Vec<Vec2<f32>>, vertex: Vec2<f32>, direction: Vec2<f32>, width: f32) {
            let normal = direction.rotate_90();
            let len = normal.len();
            let normal = if len.approx_eq(&0.0) {
                Vec2::ZERO
            } else {
                normal / len
            };
            let shift = normal * width / 2.0;
            polygon.push(vertex + shift);
            polygon.push(vertex - shift);
        }

        // Start
        add(
            &mut polygon,
            self.vertices[0],
            self.vertices[1] - self.vertices[0],
            self.width,
        );

        // Middle
        for ((prev, current), next) in self
            .vertices
            .iter()
            .copied()
            .zip(self.vertices.iter().copied().skip(1))
            .zip(self.vertices.iter().copied().skip(2))
        {
            add(&mut polygon, current, next - prev, self.width);
        }

        // End
        add(
            &mut polygon,
            self.vertices[len - 1],
            self.vertices[len - 1] - self.vertices[len - 2],
            self.width,
        );

        polygon
    }

    pub fn draw_2d(
        &self,
        geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
        camera: &impl geng::AbstractCamera2d,
    ) {
        #![allow(deprecated)]
        geng.draw_2d_helper().draw(
            framebuffer,
            camera,
            &self.triangle_strip(),
            self.color,
            ugli::DrawMode::TriangleStrip,
        );
    }
}

pub struct Segment {
    pub start: Vec2<f32>,
    pub end: Vec2<f32>,
    pub width: f32,
    pub color: Color<f32>,
}

impl From<Segment> for Chain {
    fn from(segment: Segment) -> Self {
        Self {
            vertices: vec![segment.start, segment.end],
            width: segment.width,
            color: segment.color,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_take() {
        let chain = Chain {
            vertices: vec![
                vec2(0.0, 0.0),
                vec2(1.0, 0.0),
                vec2(1.0, 1.0),
                vec2(0.0, 1.0),
            ],
            width: 1.0,
            color: Color::WHITE,
        };

        assert_eq!(
            chain.clone().take_range_length(1.0..=2.0).vertices,
            vec![vec2(1.0, 0.0), vec2(1.0, 1.0)]
        );

        assert_eq!(
            chain.clone().take_range_length(0.5..=2.75).vertices,
            vec![
                vec2(0.5, 0.0),
                vec2(1.0, 0.0),
                vec2(1.0, 1.0),
                vec2(0.25, 1.0)
            ]
        );

        assert_eq!(
            chain.clone().take_range_ratio(0.0..=1.0).vertices,
            chain.vertices
        );

        assert_eq!(
            chain
                .clone()
                .take_range_length(0.0..=chain.length())
                .vertices,
            chain.vertices
        );
    }
}
