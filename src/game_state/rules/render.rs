use super::*;

impl Rules {
    fn render(&mut self, selection: &Option<RuleSelection>) {
        self.rules
            .iter()
            .zip(self.cameras.iter())
            .zip(self.textures.iter_mut())
            .enumerate()
            .for_each(|(rule_index, ((rule, camera), texture))| {
                let texture_color = match self.focused_rule {
                    Some(index) if index == rule_index => RULE_SELECTION_COLOR,
                    _ => Color::BLACK,
                };
                texture.background_color = texture_color;

                let mut temp_framebuffer = ugli::Framebuffer::new_color(
                    self.geng.ugli(),
                    ugli::ColorAttachment::Texture(&mut texture.inner),
                );
                ugli::clear(&mut temp_framebuffer, Some(texture_color), None);

                let mut selected = Vec::new();
                if let Some(selection) = &selection {
                    if selection.rule() == rule_index {
                        selected.push(*selection.current().unwrap());
                    }
                }
                draw::graph::draw_graph(
                    &self.geng,
                    &self.assets,
                    self.geng.default_font(),
                    &mut temp_framebuffer,
                    camera,
                    rule.graph(),
                    texture.background_color,
                    Some(&selected),
                );
            })
    }

    pub fn draw(
        &mut self,
        selection: &Option<RuleSelection>,
        camera: &Camera2d,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let line_width = camera_view(camera, framebuffer.size().map(|x| x as f32)).height()
            * RULE_SEPARATION_WIDTH_FRAC;

        self.render(selection);
        let layout = layout(
            vec2(0.0, self.scroll_offset),
            self.width,
            self.rules_count(),
            camera,
            framebuffer.size().map(|x| x as f32),
        )
        .collect::<Vec<_>>();

        if let Some(rule_aabb) = layout.last() {
            // Separation line
            Chain {
                vertices: vec![rule_aabb.bottom_left(), rule_aabb.bottom_right()],
                width: line_width,
                color: RULE_SEPARATION_COLOR,
            }
            .draw_2d(&self.geng, framebuffer, camera);
        }

        for (rule_aabb, rule_texture) in layout.into_iter().zip(self.textures.iter()) {
            // Separation line
            Chain {
                vertices: vec![rule_aabb.top_left(), rule_aabb.top_right()],
                width: line_width,
                color: RULE_SEPARATION_COLOR,
            }
            .draw_2d(&self.geng, framebuffer, camera);

            // Render texture to the dedicated part on the screen
            self.geng.draw_2d(
                framebuffer,
                camera,
                &draw_2d::TexturedQuad::new(rule_aabb, &rule_texture.inner),
            );
        }
    }
}
