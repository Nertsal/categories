use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FocusedCategory {
    Rule { index: usize },
    Fact,
    Goal,
}

impl GameState {
    /// Updates the focus.
    pub fn focus(&mut self, mouse_position: Vec2<f64>) {
        let focus = self.focused_graph(mouse_position);
        self.focused_category = focus.unwrap_or(FocusedCategory::Fact);
    }

    /// Returns the focused camera.
    pub fn focused_camera(&self) -> &Camera2d {
        match &self.focused_category {
            FocusedCategory::Rule { index } => &self.rules[*index].category.camera,
            FocusedCategory::Fact => &self.fact_category.camera,
            FocusedCategory::Goal => &self.goal_category.camera,
        }
    }

    /// Returns the focused camera.
    pub fn focused_camera_mut(&mut self) -> &mut Camera2d {
        match &self.focused_category {
            FocusedCategory::Rule { index } => &mut self.rules[*index].category.camera,
            FocusedCategory::Fact => &mut self.fact_category.camera,
            FocusedCategory::Goal => &mut self.goal_category.camera,
        }
    }

    fn focused_graph(&self, mouse_position: Vec2<f64>) -> Option<FocusedCategory> {
        let mouse_pos = mouse_position.map(|x| x as f32);
        let world_pos = self
            .ui_camera
            .screen_to_world(self.state.framebuffer_size, mouse_pos);

        self.state
            .graphs_layout
            .iter()
            .find(|(_, aabb)| aabb.contains(world_pos))
            .map(|(graph, _)| *graph)
    }
}
