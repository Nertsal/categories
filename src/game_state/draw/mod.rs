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
            &self.force_graph,
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
        for (edge, from, to) in self.selection.edges.iter().filter_map(|edge| {
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
                            .map(|to| (arrow, from, to))
                    })
            })
        }) {
            draw_chain(
                self.geng.draw_2d(),
                framebuffer,
                &self.camera,
                Chain {
                    vertices: vec![from.body.position, to.body.position],
                    width: edge.edge.width + SELECTED_RADIUS,
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
                let mut points = graph.graph.vertices.iter().map(|(_, point)| point);
                points
                    .next()
                    .map(|head| (head, points))
                    .map(|(head, tail)| {
                        let mut pos_min = head.body.position;
                        let mut pos_max = head.body.position;
                        for point in tail {
                            pos_min.x = pos_min.x.min(point.body.position.x);
                            pos_min.y = pos_min.y.min(point.body.position.y);
                            pos_max.x = pos_max.x.max(point.body.position.x);
                            pos_max.y = pos_max.y.max(point.body.position.y);
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
        for (_, arrow) in graph.graph.edges.iter() {
            if let Some((from, to)) = self
                .force_graph
                .graph
                .vertices
                .get(&arrow.edge.from)
                .and_then(|from| {
                    graph
                        .graph
                        .vertices
                        .get(&arrow.edge.to)
                        .map(|to| (from, to))
                })
            {
                let to_position = to.body.position * scale;
                let from_position = from.body.position * scale;
                let delta = to_position - from_position;
                let delta_len = delta.len();
                let direction_norm = delta / delta_len;
                let start =
                    from_position + direction_norm * from.vertex.radius * scale_min + offset;
                let end = to_position - direction_norm * to.vertex.radius * scale_min + offset;
                let normal = direction_norm.rotate_90();
                let head_length = direction_norm
                    * ARROW_HEAD_LENGTH.min((end - start).len() * ARROW_LENGTH_MAX_FRAC)
                    * scale;
                let head_width = normal * ARROW_HEAD_WIDTH * scale;
                let head = end - head_length;

                // Line body
                match arrow.edge.connection {
                    ArrowConnection::Solid => {
                        draw_chain(
                            draw,
                            framebuffer,
                            &self.camera,
                            Chain {
                                vertices: vec![start, arrow.body.position, head],
                                width: arrow.edge.width,
                            },
                            arrow.edge.color,
                        );
                    }
                    ArrowConnection::Dashed => {
                        draw_dashed_chain(
                            draw,
                            framebuffer,
                            &self.camera,
                            Chain {
                                vertices: vec![start, arrow.body.position, head],
                                width: arrow.edge.width,
                            },
                            arrow.edge.color,
                        );
                    }
                }

                // Line head
                draw.draw(
                    framebuffer,
                    &self.camera,
                    &[end, head + head_width, head - head_width],
                    arrow.edge.color,
                    ugli::DrawMode::Triangles,
                );
            } else {
                warn!("Edge connects a non-existent vertex, edge = {:?}", arrow);
            }
        }

        // Vertices
        for (_, vertex) in graph.graph.vertices.iter() {
            draw.circle(
                framebuffer,
                &self.camera,
                vertex.body.position * scale + offset,
                vertex.vertex.radius * scale_min,
                vertex.vertex.color,
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
    for segment in chain.segments() {
        draw_dashed_segment(draw_2d, framebuffer, camera, segment, color);
    }
}

fn draw_dashed_segment(
    draw_2d: &Rc<geng::Draw2D>,
    framebuffer: &mut ugli::Framebuffer,
    camera: &Camera2d,
    segment: Segment,
    color: Color<f32>,
) {
    let dash_length = ARROW_DASHED_DASH_LENGTH + ARROW_DASHED_SPACE_LENGTH;
    let delta = segment.end - segment.start;
    let delta_len = delta.len();
    let direction_norm = delta / delta_len;
    let dashes = ((delta_len / dash_length).floor() as usize).max(1);
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

    draw_chain(
        draw_2d,
        framebuffer,
        camera,
        Chain {
            vertices: vec![
                segment.start + direction_norm * (dashes - 1) as f32 / dashes as f32 * delta_len,
                segment.end,
            ],
            width: segment.width,
        },
        color,
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
