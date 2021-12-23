use super::*;

impl GameState {
    pub fn update_impl(&mut self, delta_time: f32) {
        // Update focus
        self.focus(self.geng.window().cursor_position());

        // Update graphs
        self.main_graph.graph.update(delta_time);
        self.goal_graph.graph.update(delta_time);
        for rule in &mut self.rules {
            rule.update_graph(delta_time);
        }

        // Mouse update
        self.drag_update();
    }
}
