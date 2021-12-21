use super::*;

impl GameState {
    pub fn update_impl(&mut self, delta_time: f32) {
        // Resize
        self.rules.width =
            util::camera_view(&self.camera, self.framebuffer_size).width() * RULES_WIDTH_FRAC;

        // Update graphs
        self.main_graph.update(delta_time);
        self.rules.update(delta_time);

        // Mouse update
        self.drag_update();
    }
}
