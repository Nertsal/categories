use super::*;

pub fn draw_graph(
    draw_2d: &Rc<geng::Draw2D>,
    font: &Rc<geng::Font>,
    framebuffer: &mut ugli::Framebuffer,
    camera: &Camera2d,
    graph: &Graph,
    background_color: Color<f32>,
    selected_vertices: Option<&Vec<VertexId>>,
) {
    // Edges
    for (_, arrow) in graph.graph.edges.iter() {
        if let Some((from, to)) = graph.graph.vertices.get(&arrow.edge.from).and_then(|from| {
            graph
                .graph
                .vertices
                .get(&arrow.edge.to)
                .map(|to| (from, to))
        }) {
            let start = from.body.position;
            let end = to.body.position;

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
            let chain_len = chain.length();

            let end_direction = chain.end_direction().unwrap();
            let direction_norm = end_direction.normalize_or_zero();
            let normal = direction_norm.rotate_90();
            let scale =
                ARROW_HEAD_LENGTH.min(chain_len * ARROW_LENGTH_MAX_FRAC) / ARROW_HEAD_LENGTH;
            let head_length = ARROW_HEAD_LENGTH * scale;
            let head_offset = direction_norm * (head_length + to.vertex.radius);
            let head = end - head_offset;
            let head_width = normal * ARROW_HEAD_WIDTH * scale;

            let mut chain = chain.take_range_ratio(
                from.vertex.radius / chain_len..=1.0 - (to.vertex.radius + head_length) / chain_len,
            );

            // Outline
            let width = chain.width;
            chain.width += CHAIN_OUTLINE_WIDTH;
            draw_chain(draw_2d, framebuffer, camera, &chain, background_color);
            chain.width = width;

            match arrow.edge.connection {
                ArrowConnection::Best | ArrowConnection::Regular => {
                    draw_chain(draw_2d, framebuffer, camera, &chain, arrow.edge.color());
                }
                ArrowConnection::Unique => {
                    draw_dashed_chain(draw_2d, framebuffer, camera, &chain, arrow.edge.color());
                }
            }

            // Line head
            draw_2d.draw(
                framebuffer,
                camera,
                &[
                    end - direction_norm * to.vertex.radius,
                    head + head_width,
                    head - head_width,
                ],
                arrow.edge.color(),
                ugli::DrawMode::Triangles,
            );

            // Label
            if let Some(center) = arrow.bodies.get(arrow.bodies.len() / 2) {
                font.draw(
                    framebuffer,
                    camera,
                    &arrow.edge.label,
                    center.position,
                    geng::TextAlign::CENTER,
                    ARROW_LABEL_FONT_SIZE,
                    Color::GRAY,
                );
            }
        } else {
            warn!("Edge connects a non-existent vertex, edge = {:?}", arrow);
        }
    }

    // Selection
    if let Some(selection) = selected_vertices {
        for vertex in selection {
            let vertex = graph.graph.vertices.get(vertex).unwrap();
            draw_2d.circle(
                framebuffer,
                camera,
                vertex.body.position,
                vertex.vertex.radius + SELECTED_RADIUS,
                SELECTED_COLOR,
            );
        }
    }

    // Vertices
    for (_, vertex) in graph.graph.vertices.iter() {
        // Vertex
        draw_2d.circle(
            framebuffer,
            camera,
            vertex.body.position,
            vertex.vertex.radius,
            vertex.vertex.color,
        );

        // Label
        draw_fit_text(
            font,
            framebuffer,
            camera,
            &vertex.vertex.label,
            vertex.body.position,
            geng::TextAlign::CENTER,
            vertex.vertex.radius * 1.5,
            Some(vertex.vertex.radius * 1.5),
            Color::GRAY,
        );
    }
}

fn draw_fit_text(
    font: &geng::Font,
    framebuffer: &mut ugli::Framebuffer,
    camera: &impl geng::AbstractCamera2d,
    text: &str,
    mut pos: Vec2<f32>,
    align: geng::TextAlign,
    fit_width: f32,
    fit_height: Option<f32>,
    color: Color<f32>,
) {
    let mut size = 10.0;
    let aabb = font.measure(text, size);

    let width = aabb.width();
    if width.approx_eq(&0.0) {
        return;
    }

    let mut scale = fit_width / width;

    if let Some(fit_height) = fit_height {
        let height = aabb.height();
        scale = scale.min(fit_height / height);
    }

    size *= scale;
    pos.y -= size / 4.0; // Align vertically
    font.draw(framebuffer, camera, text, pos, align, size, color);
}
