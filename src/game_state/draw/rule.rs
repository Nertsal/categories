use super::*;

impl GameState {
    pub fn draw_rules(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let camera_view = camera_view(&self.camera, self.framebuffer_size);

        // Separation line
        let line_offset = vec2(self.rules.width + RULES_SECTION_SEPARATION_WIDTH, 0.0);
        draw_chain(
            self.geng.draw_2d(),
            framebuffer,
            &self.camera,
            &Chain {
                vertices: vec![
                    camera_view.top_right() - line_offset,
                    camera_view.bottom_right() - line_offset,
                ],
                width: RULES_SECTION_SEPARATION_WIDTH,
            },
            RULES_SECTION_SEPARATION_COLOR,
        );
        self.rules.draw(&self.camera, framebuffer);
    }
}
