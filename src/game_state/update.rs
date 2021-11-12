use super::*;

impl GameState {
    pub fn update_impl(&mut self, delta_time: f32) {
        // Update graphs
        self.force_graph.update(delta_time);
        self.rules.update(delta_time);

        // Focus
        let focused = self.focused_rule();
        self.rules.focus(focused);

        // Drag
        if let Some(dragging) = &mut self.dragging {
            let mouse_position = self.geng.window().mouse_pos();
            match &dragging.action {
                DragAction::Move { target } => {
                    let target_pos = match target {
                        DragTarget::Vertex { id } => self
                            .force_graph
                            .graph
                            .vertices
                            .get_mut(id)
                            .map(|vertex| &mut vertex.body.position),
                        DragTarget::Edge { id } => self
                            .force_graph
                            .graph
                            .edges
                            .get_mut(id)
                            .map(|edge| &mut edge.get_center_mut().unwrap().position),
                    };
                    match target_pos {
                        Some(target_pos) => {
                            let world_pos = self.camera.screen_to_world(
                                self.framebuffer_size,
                                mouse_position.map(|x| x as f32),
                            );
                            *target_pos = world_pos;
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

        self.rules
            .layout(&self.camera, self.framebuffer_size)
            .enumerate()
            .find(|(_, rule_aabb)| rule_aabb.contains(world_pos))
            .map(|(index, _)| index)
    }
}
