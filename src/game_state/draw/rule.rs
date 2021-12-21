use geng::Draw2d;

use super::*;

impl GameState {
    pub fn draw_rules(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let camera_view = util::camera_view(&self.camera, self.framebuffer_size);
        let line_width = camera_view.height() * RULES_SECTION_SEPARATION_WIDTH_FRAC;

        // Separation line
        let line_offset = vec2(self.rules.width + line_width / 2.0, 0.0);
        draw_2d::Chain::new(
            Chain::new(vec![
                camera_view.top_left() + line_offset,
                camera_view.bottom_left() + line_offset,
            ]),
            line_width,
            RULES_SECTION_SEPARATION_COLOR,
            1,
        )
        .draw_2d(&self.geng, framebuffer, &self.camera);
        self.rules.draw(&self.selection, &self.camera, framebuffer);
    }
}
