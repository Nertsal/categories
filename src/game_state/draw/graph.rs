use super::*;

impl GameState {
    pub(super) fn draw_graph(
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
                let direction_norm = if delta_len.approx_eq(&0.0) {
                    Vec2::ZERO
                } else {
                    delta / delta_len
                };
                let start =
                    from_position + direction_norm * from.vertex.radius * scale_min + offset;
                let end = to_position - direction_norm * to.vertex.radius * scale_min + offset;

                // Line body
                let chain = if arrow.bodies.len() > 1 {
                    CardinalSpline::new(
                        {
                            let mut bodies = vec![start];
                            bodies.extend(arrow.bodies.iter().map(|body| body.position));
                            bodies.push(end);
                            bodies
                        },
                        0.5,
                    )
                    .chain(CURVE_RESOLUTION, ARROW_WIDTH)
                } else {
                    ParabolaCurve::new([start, arrow.bodies[0].position, end])
                        .chain(CURVE_RESOLUTION, ARROW_WIDTH)
                };
                let end_direction = chain.end_direction().unwrap();
                match arrow.edge.connection {
                    ArrowConnection::Best => {
                        draw_chain(draw, framebuffer, &self.camera, chain, arrow.edge.color());
                    }
                    ArrowConnection::Regular => {
                        draw_chain(draw, framebuffer, &self.camera, chain, arrow.edge.color());
                    }
                    ArrowConnection::Unique => {
                        draw_dashed_chain(
                            draw,
                            framebuffer,
                            &self.camera,
                            chain,
                            arrow.edge.color(),
                        );
                    }
                }

                let direction_norm = end_direction.normalize();
                let normal = direction_norm.rotate_90();
                let scale = ARROW_HEAD_LENGTH.min((end - start).len() * ARROW_LENGTH_MAX_FRAC)
                    / ARROW_HEAD_LENGTH
                    * scale;
                let head_length = direction_norm * ARROW_HEAD_LENGTH * scale;
                let head = end - head_length;
                let head_width = normal * ARROW_HEAD_WIDTH * scale;

                // Line head
                draw.draw(
                    framebuffer,
                    &self.camera,
                    &[end, head + head_width, head - head_width],
                    arrow.edge.color(),
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
