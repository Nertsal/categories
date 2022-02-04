use geng::draw_2d::Draw2d;

use super::*;

pub fn morphism_color<O, M>(tag: &Option<MorphismTag<O, M>>) -> Color<f32> {
    match tag {
        Some(MorphismTag::Unique) => ARROW_UNIQUE_COLOR,
        Some(MorphismTag::Isomorphism(_, _)) => ARROW_ISOMORPHISM_COLOR,
        _ => ARROW_REGULAR_COLOR,
    }
}

pub fn draw_category(
    geng: &Geng,
    assets: &Rc<Assets>,
    font: &Rc<geng::Font>,
    framebuffer: &mut ugli::Framebuffer,
    camera: &Camera2d,
    category: &Category,
    background_color: Color<f32>,
    selection: Option<&Vec<CategoryThing>>,
) {
    // Selection
    let mut selected_vertices = HashSet::new();
    let mut selected_edges = HashSet::new();
    if let Some(selection) = selection {
        for selection in selection {
            match selection {
                CategoryThing::Object { id } => {
                    selected_vertices.insert(id);
                }
                CategoryThing::Morphism { id } => {
                    selected_edges.insert(id);
                }
            }
        }
    }

    // Morphisms
    for (id, morphism) in category.morphisms.iter() {
        draw_morphism(
            geng,
            font,
            &assets,
            framebuffer,
            camera,
            background_color,
            category,
            morphism,
            selected_edges.contains(id),
        );
    }

    // Objects
    for (id, object) in category.objects.iter() {
        draw_object(
            geng,
            font,
            framebuffer,
            camera,
            object,
            background_color,
            selected_vertices.contains(id),
        );
    }

    let framebuffer_size = framebuffer.size().map(|x| x as f32);
    let height = constants::EQUALITY_FONT_SIZE_FRAC * framebuffer_size.x;
    let offset = vec2(height / 2.0, height / 2.0);

    // Equalities
    for (i, equality) in category.equalities.all_equalities().enumerate() {
        let pos = framebuffer_size - offset - vec2(0.0, i as f32 * height * 1.5);

        let get = |edge| {
            let label = &category.morphisms.get(edge).unwrap().inner.label;
            if label.is_empty() {
                format!("[{}]", edge.raw())
            } else {
                label.to_owned()
            }
        };

        let text = format!("{} = {}", get(&equality.0), get(&equality.1));
        draw_2d::Text::unit(font.clone(), text, constants::EQUALITY_FONT_COLOR)
            .fit_into(AABB::ZERO.extend_positive(vec2(framebuffer_size.x, height)))
            .align_bounding_box(vec2(1.0, 1.0))
            .translate(pos)
            .draw_2d(geng, framebuffer, &geng::PixelPerfectCamera);
    }
}

fn draw_object(
    geng: &Geng,
    font: &Rc<geng::Font>,
    framebuffer: &mut ugli::Framebuffer,
    camera: &Camera2d,
    object: &Object,
    background_color: Color<f32>,
    is_selected: bool,
) {
    // Selection
    if is_selected {
        draw_2d::Ellipse::circle(
            object.inner.position,
            object.inner.radius + SELECTED_RADIUS,
            SELECTED_COLOR,
        )
        .draw_2d(geng, framebuffer, camera);
    }

    // Outline
    draw_2d::Ellipse::circle_with_cut(
        object.inner.position,
        object.inner.radius - POINT_OUTLINE_WIDTH,
        object.inner.radius,
        object.inner.color,
    )
    .draw_2d(geng, framebuffer, camera);

    // Background
    draw_2d::Ellipse::circle(
        object.inner.position,
        object.inner.radius - POINT_OUTLINE_WIDTH,
        background_color,
    )
    .draw_2d(geng, framebuffer, camera);

    // Label
    draw_2d::Text::unit(
        font.clone(),
        object.inner.label.to_owned(),
        object.inner.color,
    )
    .fit_into(Ellipse::circle(
        object.inner.position,
        (object.inner.radius - POINT_OUTLINE_WIDTH) * 0.8,
    ))
    .draw_2d(geng, framebuffer, camera);
}

fn draw_morphism(
    geng: &Geng,
    font: &Rc<geng::Font>,
    assets: &Rc<Assets>,
    framebuffer: &mut ugli::Framebuffer,
    camera: &Camera2d,
    background_color: Color<f32>,
    category: &Category,
    morphism: &Morphism,
    is_selected: bool,
) {
    // Find endpoints
    let (from, to, isomorphism) = match morphism.connection {
        MorphismConnection::Regular { from, to } => (from, to, false),
        MorphismConnection::Isomorphism(a, b) => (a, b, true),
    };

    let from = match category.objects.get(&from) {
        Some(from) => from,
        None => {
            warn!("An object {from:?} is connected but does not exist");
            return;
        }
    };
    let to = match category.objects.get(&to) {
        Some(to) => to,
        None => {
            warn!("An object {to:?} is connected but does not exist");
            return;
        }
    };

    let start = from.inner.position;
    let end = to.inner.position;

    // Line body
    let chain = if morphism.inner.positions.len() == 1 {
        Trajectory::parabola([start, morphism.inner.positions[0], end], -1.0..=1.0)
            .chain(CURVE_RESOLUTION)
    } else if morphism.inner.positions.len() > 1 {
        CardinalSpline::new(
            {
                let mut bodies = vec![start];
                bodies.extend(morphism.inner.positions.iter().copied());
                bodies.push(end);
                bodies
            },
            0.5,
        )
        .chain(CURVE_RESOLUTION)
    } else {
        info!("A morphism has 0 internal positions");
        return;
    };
    let chain_len = chain.length();

    let scale = ARROW_HEAD_LENGTH.min(chain_len * ARROW_LENGTH_MAX_FRAC) / ARROW_HEAD_LENGTH;
    let head_length = ARROW_HEAD_LENGTH * scale;

    let (min, max) = if isomorphism {
        (
            from.inner.radius / chain_len,
            1.0 - to.inner.radius / chain_len,
        )
    } else {
        (
            from.inner.radius / chain_len,
            1.0 - (to.inner.radius + head_length) / chain_len,
        )
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

    if morphism
        .tags
        .iter()
        .any(|tag| matches!(tag, MorphismTag::Unique))
    {
        draw_dashed_chain(
            geng,
            framebuffer,
            camera,
            &chain,
            ARROW_WIDTH,
            morphism.inner.color,
        );
    } else {
        draw_2d::Chain::new(chain, ARROW_WIDTH, morphism.inner.color, 1).draw_2d(
            geng,
            framebuffer,
            camera,
        );
    }

    // Line head
    let direction_norm = head_direction.normalize_or_zero();
    let normal = direction_norm.rotate_90();
    let head_offset = direction_norm * (head_length + to.inner.radius);
    let head = end - head_offset;
    let head_width = normal * ARROW_HEAD_WIDTH * scale;

    if !isomorphism {
        draw_2d::Polygon::new(
            vec![
                end - direction_norm * to.inner.radius,
                head + head_width,
                head - head_width,
            ],
            morphism.inner.color,
        )
        .draw_2d(geng, framebuffer, camera)
    }

    if let Some(&center) = morphism
        .inner
        .positions
        .get(morphism.inner.positions.len() / 2)
    {
        // Label
        draw_2d::Text::unit(font.clone(), morphism.inner.label.to_owned(), Color::GRAY)
            .fit_into(AABB::point(center).extend_uniform(ARROW_LABEL_FONT_SIZE))
            .draw_2d(geng, framebuffer, camera);

        // Isomorphism
        if isomorphism {
            draw_2d::Ellipse::circle(center, ARROW_ICON_RADIUS, morphism.inner.color).draw_2d(
                geng,
                framebuffer,
                camera,
            );
            draw_2d::Ellipse::circle(
                center,
                ARROW_ICON_RADIUS - ARROW_ICON_OUTLINE_WIDTH,
                background_color,
            )
            .draw_2d(geng, framebuffer, camera);

            draw_2d::TexturedQuad::colored(
                AABB::point(center).extend_uniform(ARROW_ICON_RADIUS),
                &assets.isomorphism,
                morphism.inner.color,
            )
            .draw_2d(geng, framebuffer, camera);
        }
    }
}
