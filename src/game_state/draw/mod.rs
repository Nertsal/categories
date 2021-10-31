use super::*;

mod segment;

use segment::*;

const ARROW_HEAD_WIDTH: f32 = 0.5;
const ARROW_HEAD_LENGTH: f32 = 2.0;
const ARROW_LENGTH_MAX_FRAC: f32 = 0.5;

const ARROW_DASHED_DASH_LENGTH: f32 = 0.7;
const ARROW_DASHED_SPACE_LENGTH: f32 = 0.5;

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
            &self.graph,
            vec2(0.0, 0.0),
            None,
            GraphRender::ScaledAligned {
                scale: vec2(1.0, 1.0),
            },
        );

        // Dragging
        if let Some(dragging) = &self.dragging {
            let world_pos = self.camera.screen_to_world(
                self.framebuffer_size,
                self.geng.window().mouse_pos().map(|x| x as f32),
            );
            self.geng.draw_2d().quad(
                framebuffer,
                &self.camera,
                AABB::from_corners(dragging.world_start_pos, world_pos),
                SELECTION_COLOR,
            );
        }

        // Selection
        for vertex in self
            .selection
            .vertices
            .iter()
            .filter_map(|vertex| self.graph.vertices.get(vertex))
        {
            self.geng.draw_2d().circle(
                framebuffer,
                &self.camera,
                vertex.position,
                vertex.radius + SELECTED_RADIUS,
                SELECTED_COLOR,
            )
        }
        for (edge, from, to) in self.selection.edges.iter().filter_map(|edge| {
            self.graph.edges.get(edge).and_then(|arrow| {
                self.graph.vertices.get(&arrow.from).and_then(|from| {
                    self.graph
                        .vertices
                        .get(&arrow.to)
                        .map(|to| (arrow, from, to))
                })
            })
        }) {
            draw_segment(
                self.geng.draw_2d(),
                framebuffer,
                &self.camera,
                Segment {
                    start: from.position,
                    end: to.position,
                    width: edge.width + SELECTED_RADIUS,
                },
                SELECTED_COLOR,
            );
        }
    }

    fn draw_graph(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        graph: &Graph,
        offset: Vec2<f32>,
        align: Option<Vec2<f32>>,
        render: GraphRender,
    ) {
        let draw = self.geng.draw_2d();

        // Alignment
        let align_offset = align
            .and_then(|align| {
                let mut points = graph.vertices.iter().map(|(_, point)| point);
                points
                    .next()
                    .map(|head| (head, points))
                    .map(|(head, tail)| {
                        let mut pos_min = head.position;
                        let mut pos_max = head.position;
                        for point in tail {
                            pos_min.x = pos_min.x.min(point.position.x);
                            pos_min.y = pos_min.y.min(point.position.y);
                            pos_max.x = pos_max.x.max(point.position.x);
                            pos_max.y = pos_max.y.max(point.position.y);
                        }
                        (align, pos_min, pos_max)
                    })
            })
            .map(|(align, pos_min, pos_max)| {
                let size = pos_max - pos_min;
                (size * (vec2(1.0, 1.0) - align), size)
            });

        let scale = match render {
            GraphRender::ScaledAligned { scale } => scale,
            GraphRender::Fit { size, scale } => align_offset
                .map(|(_, graph_size)| match scale {
                    GraphFitScale::Fit => size / graph_size,
                    GraphFitScale::KeepRatio => {
                        let scale = size / graph_size;
                        let scale = scale.x.min(scale.y);
                        vec2(scale, scale)
                    }
                })
                .unwrap_or(vec2(1.0, 1.0)),
        };

        let offset = match align_offset {
            Some((graph_offset, _)) => offset + graph_offset * scale,
            None => offset,
        };

        let scale_min = scale.x.min(scale.y);

        // Edges
        for (_, arrow) in graph.edges.iter() {
            if let Some((from, to)) = self
                .graph
                .vertices
                .get(&arrow.from)
                .and_then(|from| graph.vertices.get(&arrow.to).map(|to| (from, to)))
            {
                let to_position = to.position * scale;
                let from_position = from.position * scale;
                let delta = to_position - from_position;
                let delta_len = delta.len();
                let direction_norm = delta / delta_len;
                let start = from_position + direction_norm * from.radius * scale_min + offset;
                let end = to_position - direction_norm * to.radius * scale_min + offset;
                let normal = direction_norm.rotate_90();
                let head_length = direction_norm
                    * ARROW_HEAD_LENGTH.min((end - start).len() * ARROW_LENGTH_MAX_FRAC)
                    * scale;
                let head_width = normal * ARROW_HEAD_WIDTH * scale;
                let head = end - head_length;

                // Line body
                match arrow.connection {
                    ArrowConnection::Solid => {
                        draw_segment(
                            draw,
                            framebuffer,
                            &self.camera,
                            Segment {
                                start,
                                end: head,
                                width: arrow.width,
                            },
                            arrow.color,
                        );
                    }
                    ArrowConnection::Dashed => {
                        let dash_length = ARROW_DASHED_DASH_LENGTH + ARROW_DASHED_SPACE_LENGTH;
                        let delta_len = (head - start).len();
                        let dashes = (delta_len / dash_length).floor() as usize;
                        for i in 0..(dashes - 1) {
                            let dash_start =
                                start + direction_norm * i as f32 / dashes as f32 * delta_len;
                            draw_segment(
                                draw,
                                framebuffer,
                                &self.camera,
                                Segment {
                                    start: dash_start,
                                    end: dash_start + direction_norm * ARROW_DASHED_DASH_LENGTH,
                                    width: arrow.width,
                                },
                                arrow.color,
                            );
                        }

                        draw_segment(
                            draw,
                            framebuffer,
                            &self.camera,
                            Segment {
                                start: start
                                    + direction_norm * (dashes - 1) as f32 / dashes as f32
                                        * delta_len,
                                end: head,
                                width: arrow.width,
                            },
                            arrow.color,
                        );
                    }
                }

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
        for (_, vertex) in graph.vertices.iter() {
            draw.circle(
                framebuffer,
                &self.camera,
                vertex.position * scale + offset,
                vertex.radius * scale_min,
                vertex.color,
            );
        }
    }
}

fn draw_segment(
    draw_2d: &Rc<geng::Draw2D>,
    framebuffer: &mut ugli::Framebuffer,
    camera: &Camera2d,
    segment: Segment,
    color: Color<f32>,
) {
    draw_2d.draw(
        framebuffer,
        camera,
        &segment.polygon(),
        color,
        ugli::DrawMode::TriangleFan,
    );
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
