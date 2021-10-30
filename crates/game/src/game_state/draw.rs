use super::*;

const ARROW_HEAD_WIDTH: f32 = 0.5;
const ARROW_HEAD_LENGTH: f32 = 3.0;
const ARROW_LENGTH_MAX_FRAC: f32 = 0.5;

impl GameState {
    pub fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        let draw = self.geng.draw_2d();

        // Edges
        for arrow in self.graph.edges.iter() {
            if let Some((from, to)) = self
                .graph
                .vertices
                .get(&arrow.from)
                .and_then(|from| self.graph.vertices.get(&arrow.to).map(|to| (from, to)))
            {
                let delta = to.position - from.position;
                let delta_len = delta.len();
                let direction = delta / delta_len;
                let start = from.position + direction * from.radius;
                let end = to.position - direction * to.radius;
                let normal = direction.rotate_90();
                let head_length =
                    direction * ARROW_HEAD_LENGTH.min(delta_len * ARROW_LENGTH_MAX_FRAC);
                let head_width = normal * ARROW_HEAD_WIDTH;
                let head = to.position - head_length;
                // Line body
                draw.draw(
                    framebuffer,
                    &self.camera,
                    &[start, end],
                    arrow.color,
                    ugli::DrawMode::LineStrip {
                        line_width: arrow.width,
                    },
                );
                // Line head
                draw.draw(
                    framebuffer,
                    &self.camera,
                    &[end, head + head_width, head - head_width],
                    arrow.color,
                    ugli::DrawMode::TriangleFan,
                );
            } else {
                warn!("Edge connects a non-existent vertex, edge = {:?}", arrow);
            }
        }

        // Vertices
        for (_, vertex) in self.graph.vertices.iter() {
            draw.circle(
                framebuffer,
                &self.camera,
                vertex.position,
                vertex.radius,
                vertex.color,
            );
        }
    }
}
