use super::*;

impl GameState {
    pub fn update_impl(&mut self, delta_time: f32) {
        // Update graphs
        self.force_graph.update(delta_time);
        for (rule, _) in &mut self.rules {
            rule.update_graph(delta_time);
        }

        // Focus
        self.focused_rule = self.focused_rule();

        // Drag
        if let Some(dragging) = &mut self.dragging {
            let mouse_position = self.geng.window().mouse_pos();
            match &dragging.action {
                DragAction::MoveVertex { vertex } => {
                    let world_pos = self
                        .camera
                        .screen_to_world(self.framebuffer_size, mouse_position.map(|x| x as f32));
                    match self.force_graph.graph.vertices.get_mut(vertex) {
                        Some(vertex) => {
                            vertex.body.position = world_pos;
                        }
                        None => {
                            self.dragging = None;
                        }
                    }
                }
                DragAction::MoveEdge { edge } => {
                    let world_pos = self
                        .camera
                        .screen_to_world(self.framebuffer_size, mouse_position.map(|x| x as f32));
                    match self.force_graph.graph.edges.get_mut(edge) {
                        Some(edge) => {
                            edge.get_center_mut().unwrap().position = world_pos;
                        }
                        None => {
                            self.dragging = None;
                        }
                    }
                }
                _ => (),
            }
        }
    }

    fn focused_rule(&self) -> Option<usize> {
        let mouse_pos = self.geng.window().mouse_pos().map(|x| x as f32);
        let world_pos = self
            .camera
            .screen_to_world(self.framebuffer_size, mouse_pos);

        self.rules_layout()
            .enumerate()
            .find(|(_, rule_aabb)| rule_aabb.contains(world_pos))
            .map(|(index, _)| index)
    }
}
