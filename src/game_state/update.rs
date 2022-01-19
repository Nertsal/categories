use super::*;

impl GameState {
    pub fn update_impl(&mut self, delta_time: f32) {
        // Update focus
        self.focus(self.geng.window().cursor_position());

        // Apply forces to objects/morphisms
        for category in vec![&mut self.fact_category, &mut self.goal_category]
            .into_iter()
            .chain(self.rules.iter_mut().map(|rule| rule.get_category_mut()))
        {
            update_category(category);
        }

        // Mouse update
        self.drag_update();
    }
}

fn update_category(category: &mut RenderableCategory) {
    todo!()
}
