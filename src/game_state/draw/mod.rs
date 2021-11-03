use super::*;

mod graph;

const ARROW_HEAD_WIDTH: f32 = 0.5;
const ARROW_HEAD_LENGTH: f32 = 2.0;
const ARROW_LENGTH_MAX_FRAC: f32 = 0.5;

const ARROW_DASHED_DASH_LENGTH: f32 = 0.7;
const ARROW_DASHED_SPACE_LENGTH: f32 = 0.5;
const ARROW_DASH_FULL_LENGTH: f32 = ARROW_DASHED_DASH_LENGTH + ARROW_DASHED_SPACE_LENGTH;

const CURVE_RESOLUTION: usize = 10;

const SELECTION_COLOR: Color<f32> = Color {
    r: 0.0,
    g: 0.0,
    b: 0.5,
    a: 0.5,
};
const SELECTED_RADIUS: f32 = 0.5;
const SELECTED_COLOR: Color<f32> = Color {
    r: 0.7,
    g: 0.7,
    b: 0.7,
    a: 0.5,
};

impl GameState {
    pub fn draw_impl(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        // Graph
        self.draw_graph(
            framebuffer,
            &self.force_graph,
            vec2(0.0, 0.0),
            None,
            GraphRender::ScaledAligned {
                scale: vec2(1.0, 1.0),
            },
        );

        // Dragging
        match &self.dragging {
            Some(Dragging::Selection {
                world_start_pos, ..
            }) => {
                let world_pos = self.camera.screen_to_world(
                    self.framebuffer_size,
                    self.geng.window().mouse_pos().map(|x| x as f32),
                );
                self.geng.draw_2d().quad(
                    framebuffer,
                    &self.camera,
                    AABB::from_corners(*world_start_pos, world_pos),
                    SELECTION_COLOR,
                );
            }
            _ => (),
        }

        // Selection
        for vertex in self
            .selection
            .vertices
            .iter()
            .filter_map(|vertex| self.force_graph.graph.vertices.get(vertex))
        {
            self.geng.draw_2d().circle(
                framebuffer,
                &self.camera,
                vertex.body.position,
                vertex.vertex.radius + SELECTED_RADIUS,
                SELECTED_COLOR,
            )
        }
        for edge_points in self.selection.edges.iter().filter_map(|edge| {
            self.force_graph.graph.edges.get(edge).and_then(|arrow| {
                self.force_graph
                    .graph
                    .vertices
                    .get(&arrow.edge.from)
                    .and_then(|from| {
                        self.force_graph
                            .graph
                            .vertices
                            .get(&arrow.edge.to)
                            .map(|to| {
                                let mut points = Vec::with_capacity(arrow.bodies.len() + 2);
                                points.push(from.body.position);
                                points.extend(arrow.bodies.iter().map(|body| body.position));
                                points.push(to.body.position);
                                points
                            })
                    })
            })
        }) {
            draw_chain(
                self.geng.draw_2d(),
                framebuffer,
                &self.camera,
                Curve::chain(
                    &CardinalSpline {
                        points: edge_points,
                        tension: 0.5,
                    },
                    CURVE_RESOLUTION,
                    ARROW_WIDTH + SELECTED_RADIUS,
                ),
                SELECTED_COLOR,
            );
        }
    }
}

fn draw_chain(
    draw_2d: &Rc<geng::Draw2D>,
    framebuffer: &mut ugli::Framebuffer,
    camera: &Camera2d,
    chain: Chain,
    color: Color<f32>,
) {
    draw_2d.draw(
        framebuffer,
        camera,
        &chain.triangle_strip(),
        color,
        ugli::DrawMode::TriangleStrip,
    );
}

fn draw_dashed_chain(
    draw_2d: &Rc<geng::Draw2D>,
    framebuffer: &mut ugli::Framebuffer,
    camera: &Camera2d,
    chain: Chain,
    color: Color<f32>,
) {
    let mut dash_full_left = 0.0;
    for segment in chain.segments() {
        let last_len =
            draw_dashed_segment(draw_2d, framebuffer, camera, segment, color, dash_full_left);
        dash_full_left = (ARROW_DASH_FULL_LENGTH - last_len).max(0.0);
    }
}

/// Draws a dashed segment.
/// Returns the unrendered length of the last dash.
fn draw_dashed_segment(
    draw_2d: &Rc<geng::Draw2D>,
    framebuffer: &mut ugli::Framebuffer,
    camera: &Camera2d,
    segment: Segment,
    color: Color<f32>,
    dash_full_left: f32,
) -> f32 {
    let delta = segment.end - segment.start;
    let delta_len = delta.len();
    let direction_norm = delta / delta_len;
    let dashes = ((delta_len / ARROW_DASH_FULL_LENGTH).floor() as usize).max(1);

    if dash_full_left > 0.0 {
        // Finish drawing the previous dash and offset current segment
        let dash_full_length = dash_full_left.min(delta_len);
        let dash_length = dash_full_left - ARROW_DASHED_SPACE_LENGTH;
        if dash_length > 0.0 {
            // Finish dash
            let dash_length = dash_length.min(dash_full_length);
            let dash_end = segment.start + direction_norm * dash_length;
            draw_chain(
                draw_2d,
                framebuffer,
                camera,
                Chain {
                    vertices: vec![segment.start, dash_end],
                    width: segment.width,
                },
                color,
            );
        }

        let dash_left = dash_full_left - dash_full_length;
        if dash_left > 0.0 {
            return dash_left;
        }

        let dash_full_end = segment.start + dash_full_length * direction_norm;
        return draw_dashed_segment(
            draw_2d,
            framebuffer,
            camera,
            Segment {
                start: dash_full_end,
                ..segment
            },
            color,
            0.0,
        );
    }

    for i in 0..(dashes - 1) {
        let dash_start = segment.start + direction_norm * i as f32 / dashes as f32 * delta_len;
        draw_chain(
            draw_2d,
            framebuffer,
            camera,
            Chain {
                vertices: vec![
                    dash_start,
                    dash_start + direction_norm * ARROW_DASHED_DASH_LENGTH,
                ],
                width: segment.width,
            },
            color,
        );
    }

    let last_start =
        segment.start + direction_norm * (dashes - 1) as f32 / dashes as f32 * delta_len;
    draw_chain(
        draw_2d,
        framebuffer,
        camera,
        Chain {
            vertices: vec![last_start, segment.end],
            width: segment.width,
        },
        color,
    );

    (ARROW_DASH_FULL_LENGTH - (segment.end - last_start).len()).max(0.0)
}

enum GraphRender {
    Fit {
        size: Vec2<f32>,
        scale: GraphFitScale,
    },
    ScaledAligned {
        scale: Vec2<f32>,
    },
}

enum GraphFitScale {
    KeepRatio,
    Fit,
}
