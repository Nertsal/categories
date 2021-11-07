use super::*;

impl GameState {
    pub fn draw_rules(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let camera_view = camera_view(&self.camera, self.framebuffer_size);

        // Separation line
        let line_offset = vec2(RULES_WIDTH + RULES_SECTION_SEPARATION_WIDTH, 0.0);
        draw_chain(
            self.geng.draw_2d(),
            framebuffer,
            &self.camera,
            Chain {
                vertices: vec![
                    camera_view.top_right() - line_offset,
                    camera_view.bottom_right() - line_offset,
                ],
                width: RULES_SECTION_SEPARATION_WIDTH,
            },
            RULES_SECTION_SEPARATION_COLOR,
        );

        let rule_height = RULES_WIDTH / RULE_RESOLUTION.x as f32 * RULE_RESOLUTION.y as f32;
        let rule_aabb_base = AABB::point(camera_view.top_right())
            .extend_left(RULES_WIDTH)
            .extend_down(rule_height);

        for (rule_index, (rule, camera)) in self.rules.iter().enumerate() {
            let rule_aabb = rule_aabb_base.translate(vec2(0.0, -rule_height * rule_index as f32));

            // Separation line
            draw_chain(
                self.geng.draw_2d(),
                framebuffer,
                &self.camera,
                Chain {
                    vertices: vec![rule_aabb.top_left(), rule_aabb.top_right()],
                    width: RULE_SEPARATION_WIDTH,
                },
                RULE_SEPARATION_COLOR,
            );

            // Render to temporary texture
            let mut texture =
                ugli::Texture2d::new_with(self.geng.ugli(), RULE_RESOLUTION, |_| Color::BLACK);
            let mut temp_framebuffer = ugli::Framebuffer::new_color(
                self.geng.ugli(),
                ugli::ColorAttachment::Texture(&mut texture),
            );
            draw_graph(
                self.geng.draw_2d(),
                self.geng.default_font(),
                &mut temp_framebuffer,
                camera,
                rule.graph(),
            );

            // Render texture to the dedicated part on the screen
            use geng::draw_2d::TexturedVertex;
            self.geng.draw_2d().draw_textured(
                framebuffer,
                &self.camera,
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
                &texture,
                Color::WHITE,
                ugli::DrawMode::TriangleFan,
            );
        }
    }
}
