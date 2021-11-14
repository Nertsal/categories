use super::*;

impl GameState {
    pub fn draw_rules(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let camera_view = camera_view(&self.camera, self.framebuffer_size);
        let line_width = camera_view.height() * RULES_SECTION_SEPARATION_WIDTH_FRAC;

        // Separation line
        let line_offset = vec2(self.rules.width + line_width / 2.0, 0.0);
        draw_chain(
            self.geng.draw_2d(),
            framebuffer,
            &self.camera,
            &Chain {
                vertices: vec![
                    camera_view.top_right() - line_offset,
                    camera_view.bottom_right() - line_offset,
                ],
                width: line_width,
            },
            RULES_SECTION_SEPARATION_COLOR,
        );
        self.rules.draw(&self.camera, framebuffer);
    }
}
