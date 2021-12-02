use geng::draw_2d::Draw2d;

use super::*;

pub fn draw_graph(
    geng: &Geng,
    assets: &Rc<Assets>,
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
        draw_edge(
            geng,
            font,
            &assets,
            framebuffer,
            camera,
            background_color,
            graph,
            edge,
            selected_edges.contains(id),
        );
    }

    // Vertices
    for (id, vertex) in graph.graph.vertices.iter() {
        draw_vertex(
            geng,
            font,
            framebuffer,
            camera,
            vertex,
            background_color,
            selected_vertices.contains(id),
        );
    }
}

fn draw_vertex(
    geng: &Geng,
    font: &Rc<geng::Font>,
    framebuffer: &mut ugli::Framebuffer,
    camera: &Camera2d,
    vertex: &ForceVertex<Point>,
    background_color: Color<f32>,
    is_selected: bool,
) {
    // Selection
    if is_selected {
        draw_2d::Ellipse::circle(
            vertex.body.position,
            vertex.vertex.radius + SELECTED_RADIUS,
            SELECTED_COLOR,
        )
        .draw_2d(geng, framebuffer, camera);
    }

    // Outline
    draw_2d::Ellipse::circle_with_cut(
        vertex.body.position,
        vertex.vertex.radius - POINT_OUTLINE_WIDTH,
        vertex.vertex.radius,
        vertex.vertex.color,
    )
    .draw_2d(geng, framebuffer, camera);

    // Background
    draw_2d::Ellipse::circle(
        vertex.body.position,
        vertex.vertex.radius - POINT_OUTLINE_WIDTH,
        background_color,
    )
    .draw_2d(geng, framebuffer, camera);

    // Label
    draw_2d::Text::unit(
        font.clone(),
        vertex.vertex.label.to_owned(),
        vertex.vertex.color,
    )
    .fit_into(Ellipse::circle(
        vertex.body.position,
        (vertex.vertex.radius - POINT_OUTLINE_WIDTH) * 0.8,
    ))
    .draw_2d(geng, framebuffer, camera);
}

fn draw_edge(
    geng: &Geng,
    font: &Rc<geng::Font>,
    assets: &Rc<Assets>,
    framebuffer: &mut ugli::Framebuffer,
    camera: &Camera2d,
    background_color: Color<f32>,
    graph: &Graph,
    edge: &ForceEdge<Arrow<VertexId>>,
    is_selected: bool,
) {
    // Find endpoints
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
        .chain(CURVE_RESOLUTION)
    } else {
        Trajectory::parabola([start, edge.bodies[0].position, end], -1.0..=1.0)
            .chain(CURVE_RESOLUTION)
    };
    let chain_len = chain.length();

    let scale = ARROW_HEAD_LENGTH.min(chain_len * ARROW_LENGTH_MAX_FRAC) / ARROW_HEAD_LENGTH;
    let head_length = ARROW_HEAD_LENGTH * scale;

    let (min, max) = match edge.edge.connection {
        ArrowConnection::Isomorphism => (
            from.vertex.radius / chain_len,
            1.0 - to.vertex.radius / chain_len,
        ),
        _ => (
            from.vertex.radius / chain_len,
            1.0 - (to.vertex.radius + head_length) / chain_len,
        ),
    };
    let chain = chain.take_range_ratio(min..=max);

    // Outline
    draw_2d::Chain::new(
        chain.clone(),
        ARROW_WIDTH + CHAIN_OUTLINE_WIDTH,
        background_color,
        1,
    )
    .draw_2d(geng, framebuffer, camera);

    if is_selected {
        // Selection
        draw_2d::Chain::new(
            chain.clone(),
            ARROW_WIDTH + SELECTED_RADIUS,
            SELECTED_COLOR,
            1,
        )
        .draw_2d(geng, framebuffer, camera);
    }

    let head_direction = end - *chain.vertices.last().unwrap();

    match edge.edge.connection {
        ArrowConnection::Best | ArrowConnection::Regular | ArrowConnection::Isomorphism => {
            draw_2d::Chain::new(chain, ARROW_WIDTH, edge.edge.color, 1).draw_2d(
                geng,
                framebuffer,
                camera,
            );
        }
        ArrowConnection::Unique => {
            draw_dashed_chain(
                geng,
                framebuffer,
                camera,
                &chain,
                ARROW_WIDTH,
                edge.edge.color,
            );
        }
    }

    // Line head
    let direction_norm = head_direction.normalize_or_zero();
    let normal = direction_norm.rotate_90();
    let head_offset = direction_norm * (head_length + to.vertex.radius);
    let head = end - head_offset;
    let head_width = normal * ARROW_HEAD_WIDTH * scale;
    match edge.edge.connection {
        ArrowConnection::Isomorphism => (),
        _ => draw_2d::Polygon::new(
            vec![
                end - direction_norm * to.vertex.radius,
                head + head_width,
                head - head_width,
            ],
            edge.edge.color,
        )
        .draw_2d(geng, framebuffer, camera),
    }

    if let Some(center) = edge.bodies.get(edge.bodies.len() / 2) {
        // Label
        draw_2d::Text::unit(font.clone(), edge.edge.label.to_owned(), Color::GRAY)
            .fit_into(AABB::point(center.position).extend_uniform(ARROW_LABEL_FONT_SIZE))
            .draw_2d(geng, framebuffer, camera);

        // Isomorphism
        if let ArrowConnection::Isomorphism = edge.edge.connection {
            draw_2d::Ellipse::circle(center.position, ARROW_ICON_RADIUS, edge.edge.color).draw_2d(
                geng,
                framebuffer,
                camera,
            );
            draw_2d::Ellipse::circle(
                center.position,
                ARROW_ICON_RADIUS - ARROW_ICON_OUTLINE_WIDTH,
                background_color,
            )
            .draw_2d(geng, framebuffer, camera);

            draw_2d::TexturedQuad::colored(
                AABB::point(center.position).extend_uniform(ARROW_ICON_RADIUS),
                &assets.isomorphism,
                edge.edge.color,
            )
            .draw_2d(geng, framebuffer, camera);
        }
    }
}
