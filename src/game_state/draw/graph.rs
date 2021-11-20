use super::*;

pub fn draw_graph(
    geng: &Geng,
    font: &Rc<geng::Font>,
    framebuffer: &mut ugli::Framebuffer,
    camera: &Camera2d,
    graph: &Graph,
    background_color: Color<f32>,
    selection: Option<&Vec<GraphObject>>,
) {
    // Selection
    let mut selected_vertices = HashSet::new();
    let mut selected_edges = HashSet::new();
    if let Some(selection) = selection {
        for selection in selection {
            match selection {
                GraphObject::Vertex { id } => {
                    selected_vertices.insert(id);
                }
                GraphObject::Edge { id } => {
                    selected_edges.insert(id);
                }
            }
        }
    }

    // Edges
    for (id, edge) in graph.graph.edges.iter() {
        // Edge
        draw_edge(
            geng,
            framebuffer,
            camera,
            background_color,
            graph,
            edge,
            selected_edges.contains(id),
        );

        // Label
        if let Some(center) = edge.bodies.get(edge.bodies.len() / 2) {
            font.draw(
                framebuffer,
                camera,
                &edge.edge.label,
                center.position,
                geng::TextAlign::CENTER,
                ARROW_LABEL_FONT_SIZE,
                Color::GRAY,
            );
        }
    }

    // Vertices
    for (id, vertex) in graph.graph.vertices.iter() {
        // Vertex
        draw_vertex(
            geng,
            framebuffer,
            camera,
            vertex,
            selected_vertices.contains(id),
        );

        // Label
        draw_fit_text(
            font,
            framebuffer,
            camera,
            &vertex.vertex.label,
            vertex.body.position,
            geng::TextAlign::CENTER,
            (vertex.vertex.radius - POINT_OUTLINE_WIDTH) * 1.5,
            Some((vertex.vertex.radius - POINT_OUTLINE_WIDTH) * 1.5),
            vertex.vertex.color,
        );
    }
}

fn draw_vertex(
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
    camera: &Camera2d,
    vertex: &ForceVertex<Point>,
    is_selected: bool,
) {
    if is_selected {
        geng.draw_2d(
            framebuffer,
            camera,
            &draw_2d::Ellipse::circle(
                vertex.body.position,
                vertex.vertex.radius + SELECTED_RADIUS,
                SELECTED_COLOR,
            ),
        );
    }

    geng.draw_2d(
        framebuffer,
        camera,
        &draw_2d::Ellipse::circle_with_cut(
            vertex.body.position,
            vertex.vertex.radius - POINT_OUTLINE_WIDTH,
            vertex.vertex.radius,
            vertex.vertex.color,
        ),
    );
}

fn draw_edge(
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
    camera: &Camera2d,
    background_color: Color<f32>,
    graph: &Graph,
    edge: &ForceEdge<Arrow<VertexId>>,
    is_selected: bool,
) {
    let (from, to) = graph
        .graph
        .vertices
        .get(&edge.edge.from)
        .and_then(|from| graph.graph.vertices.get(&edge.edge.to).map(|to| (from, to)))
        .expect(&format!(
            "Edge connects a non-existent vertex, edge = {:?}",
            edge
        ));

    let start = from.body.position;
    let end = to.body.position;

    // Line body
    let chain = if edge.bodies.len() > 1 {
        CardinalSpline::new(
            {
                let mut bodies = vec![start];
                bodies.extend(edge.bodies.iter().map(|body| body.position));
                bodies.push(end);
                bodies
            },
            0.5,
        )
        .chain(CURVE_RESOLUTION, ARROW_WIDTH, edge.edge.color)
    } else {
        ParabolaCurve::new([start, edge.bodies[0].position, end]).chain(
            CURVE_RESOLUTION,
            ARROW_WIDTH,
            edge.edge.color,
        )
    };
    let chain_len = chain.length();

    let end_direction = chain.end_direction().unwrap();
    let direction_norm = end_direction.normalize_or_zero();
    let normal = direction_norm.rotate_90();
    let scale = ARROW_HEAD_LENGTH.min(chain_len * ARROW_LENGTH_MAX_FRAC) / ARROW_HEAD_LENGTH;
    let head_length = ARROW_HEAD_LENGTH * scale;
    let head_offset = direction_norm * (head_length + to.vertex.radius);
    let head = end - head_offset;
    let head_width = normal * ARROW_HEAD_WIDTH * scale;

    let mut chain = chain.take_range_ratio(
        from.vertex.radius / chain_len..=1.0 - (to.vertex.radius + head_length) / chain_len,
    );

    // Outline
    let width = chain.width;
    let color = chain.color;
    chain.width += CHAIN_OUTLINE_WIDTH;
    chain.color = background_color;
    chain.draw_2d(geng, framebuffer, camera);
    chain.color = color;
    chain.width = width;

    if is_selected {
        // Selection
        let width = chain.width;
        let color = chain.color;
        chain.width += SELECTED_RADIUS;
        chain.color = SELECTED_COLOR;
        chain.draw_2d(geng, framebuffer, camera);
        chain.color = color;
        chain.width = width;
    }

    match edge.edge.connection {
        ArrowConnection::Best | ArrowConnection::Regular => {
            chain.draw_2d(geng, framebuffer, camera);
        }
        ArrowConnection::Unique => {
            draw_dashed_chain(geng, framebuffer, camera, &chain);
        }
    }

    // Line head
    geng.draw_2d(
        framebuffer,
        camera,
        &draw_2d::Polygon::new(
            vec![
                end - direction_norm * to.vertex.radius,
                head + head_width,
                head - head_width,
            ],
            edge.edge.color,
        ),
    );
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
    let mut size = 1000.0;
    let aabb = font.measure(text, size);

    let width = (aabb.width().sqr() + aabb.height().sqr()).sqrt();
    if width.approx_eq(&0.0) {
        return;
    }

    let mut scale = fit_width / width;

    if let Some(fit_height) = fit_height {
        let height = aabb.height();
        scale = scale.min(fit_height / height);
    }

    size *= scale;
    let aabb = aabb.map(|x| x * scale); // Alignment magic
    pos.y -= aabb.y_min + aabb.height() * 0.5; // Align vertically
    pos.x -= aabb.x_min + aabb.width() * align.0; // Align horizontally
    font.draw(
        framebuffer,
        camera,
        text,
        pos,
        geng::TextAlign::LEFT,
        size,
        color,
    );
}
