use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FocusedGraph {
    Main,
    Rule { index: usize },
}

impl FocusedGraph {
    pub fn is_main(&self) -> bool {
        match self {
            Self::Main => true,
            _ => false,
        }
    }
}

impl GameState {
    /// Returns the graph, a local position in it, and an aabb representing it
    pub fn get_graph_mut(
        &mut self,
        graph: &FocusedGraph,
        position: Vec2<f32>,
    ) -> Option<(&mut Graph, Vec2<f32>, AABB<f32>)> {
        match graph {
            FocusedGraph::Main => {
                let aabb = self.main_graph_aabb();
                Some((&mut self.main_graph, position, aabb))
            }
            FocusedGraph::Rule { index } => {
                let (_, pos, aabb) = self.world_to_rule_pos(position, *index);
                self.rules
                    .get_rule_mut(*index)
                    .map(|rule| (rule.graph_mut(), pos, aabb))
            }
        }
    }

    fn main_graph_aabb(&self) -> AABB<f32> {
        let camera_view = util::camera_view(&self.camera, self.framebuffer_size);
        camera_view.extend_left(
            -camera_view.width() * RULES_SECTION_SEPARATION_WIDTH_FRAC - self.rules.width,
        )
    }

    /// Updates the focus.
    pub fn focus(&mut self, mouse_position: Vec2<f64>) {
        let focus = self.focused_rule(mouse_position);
        self.rules.focus(focus);
        self.focused_graph = match focus {
            Some(index) => FocusedGraph::Rule { index },
            None => FocusedGraph::Main,
        };
    }

    /// Returns the focused camera.
    pub fn focused_camera(&self) -> &Camera2d {
        match &self.focused_graph {
            FocusedGraph::Rule { index } => self.rules.get_camera(*index).unwrap(),
            FocusedGraph::Main => &self.camera,
        }
    }

    /// Returns the focused camera.
    pub fn focused_camera_mut(&mut self) -> &mut Camera2d {
        match &self.focused_graph {
            FocusedGraph::Rule { index } => self.rules.get_camera_mut(*index).unwrap(),
            FocusedGraph::Main => &mut self.camera,
        }
    }

    fn focused_rule(&self, mouse_position: Vec2<f64>) -> Option<usize> {
        let mouse_pos = mouse_position.map(|x| x as f32);
        let world_pos = self
            .camera
            .screen_to_world(self.framebuffer_size, mouse_pos);

        self.rules
            .layout(&self.camera, self.framebuffer_size)
            .enumerate()
            .find(|(_, rule_aabb)| rule_aabb.contains(world_pos))
            .map(|(index, _)| index)
    }
}
