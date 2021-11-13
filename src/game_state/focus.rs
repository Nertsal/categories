use super::*;

#[derive(Debug, Clone, Copy)]
pub enum FocusedGraph {
    Main,
    Rule { index: usize },
}

impl FocusedGraph {
    pub fn is_main(&self) -> bool {
        match self {
            FocusedGraph::Main => true,
            _ => false,
        }
    }
}

impl GameState {
    /// Returns the graph and a local position in it
    pub fn get_graph_mut(
        &mut self,
        graph: &FocusedGraph,
        position: Vec2<f32>,
    ) -> Option<(&mut Graph, Vec2<f32>)> {
        match graph {
            FocusedGraph::Main => Some((&mut self.main_graph, position)),
            FocusedGraph::Rule { index } => {
                let pos = self.world_to_rule_pos(position, *index);
                self.rules
                    .get_rule_mut(*index)
                    .map(|rule| (rule.graph_mut(), pos))
            }
        }
    }

    /// Updates the focus. Returns the focused graph.
    pub fn focus(&mut self) -> &Graph {
        let focus = self.focused_rule();
        self.rules.focus(focus);
        focus
            .map(|index| {
                self.focused_graph = FocusedGraph::Rule { index };
                self.rules.get_rule(index).unwrap().graph()
            })
            .unwrap_or_else(|| {
                self.focused_graph = FocusedGraph::Main;
                &self.main_graph
            })
    }

    pub fn focused_rule(&self) -> Option<usize> {
        let mouse_pos = self.geng.window().mouse_pos().map(|x| x as f32);
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
