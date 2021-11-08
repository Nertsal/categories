use super::*;

impl Rules {
    pub fn render<'a>(&'a self) -> impl Iterator<Item = RuleTexture> + 'a {
        self.rules.iter().zip(self.cameras.iter()).enumerate().map(
            |(rule_index, (rule, camera))| {
                let texture_color = match self.focused_rule {
                    Some(index) if index == rule_index => RULE_SELECTION_COLOR,
                    _ => Color::BLACK,
                };

                let mut texture =
                    RuleTexture::new_with(self.geng.ugli(), RULE_RESOLUTION, |_| texture_color);
                let mut temp_framebuffer = ugli::Framebuffer::new_color(
                    self.geng.ugli(),
                    ugli::ColorAttachment::Texture(&mut texture),
                );
                draw::graph::draw_graph(
                    self.geng.draw_2d(),
                    self.geng.default_font(),
                    &mut temp_framebuffer,
                    camera,
                    rule.graph(),
                );
                texture
            },
        )
    }

    pub fn draw(&mut self, camera: &Camera2d, framebuffer: &mut ugli::Framebuffer) {
        for (rule_aabb, rule_texture) in layout(
            self.width,
            self.rules_count(),
            camera,
            framebuffer.size().map(|x| x as f32),
        )
        .zip(self.render())
        {
            // Separation line
            draw::chain::draw_chain(
                self.geng.draw_2d(),
                framebuffer,
                camera,
                Chain {
                    vertices: vec![rule_aabb.top_left(), rule_aabb.top_right()],
                    width: RULE_SEPARATION_WIDTH,
                },
                RULE_SEPARATION_COLOR,
            );

            // Render texture to the dedicated part on the screen
            use geng::draw_2d::TexturedVertex;
            self.geng.draw_2d().draw_textured(
                framebuffer,
                camera,
                &[
                    TexturedVertex {
                        a_pos: rule_aabb.bottom_left(),
                        a_color: Color::WHITE,
                        a_vt: vec2(0.0, 1.0),
                    },
                    TexturedVertex {
                        a_pos: rule_aabb.top_left(),
                        a_color: Color::WHITE,
                        a_vt: vec2(0.0, 0.0),
                    },
                    TexturedVertex {
                        a_pos: rule_aabb.top_right(),
                        a_color: Color::WHITE,
                        a_vt: vec2(1.0, 0.0),
                    },
                    TexturedVertex {
                        a_pos: rule_aabb.bottom_right(),
                        a_color: Color::WHITE,
                        a_vt: vec2(1.0, 1.0),
                    },
                ],
                &rule_texture,
                Color::WHITE,
                ugli::DrawMode::TriangleFan,
            );
        }
    }
}
