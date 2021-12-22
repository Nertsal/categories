use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FocusedGraph {
    Rule { index: usize },
    Main,
    Goal,
}

impl GameState {
    /// Updates the focus.
    pub fn focus(&mut self, mouse_position: Vec2<f64>) {
        let focus = self.focused_graph(mouse_position);
        self.focused_graph = focus.unwrap_or(FocusedGraph::Main);
    }

    /// Returns the focused camera.
    pub fn focused_camera(&self) -> &Camera2d {
        match &self.focused_graph {
            FocusedGraph::Rule { index } => &self.rules[*index].graph().camera,
            FocusedGraph::Main => &self.main_graph.camera,
            FocusedGraph::Goal => &self.goal_graph.camera,
        }
    }

    /// Returns the focused camera.
    pub fn focused_camera_mut(&mut self) -> &mut Camera2d {
        match &self.focused_graph {
            FocusedGraph::Rule { index } => &mut self.rules[*index].graph_mut().camera,
            FocusedGraph::Main => &mut self.main_graph.camera,
            FocusedGraph::Goal => &mut self.goal_graph.camera,
        }
    }

    fn focused_graph(&self, mouse_position: Vec2<f64>) -> Option<FocusedGraph> {
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
